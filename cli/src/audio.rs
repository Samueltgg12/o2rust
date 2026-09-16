//! Audio output: cpal stream fed by the emulator's rtrb ring.
//!
//! The core MACE audio pushes mono guest-rate `f32` frames into an [`rtrb`]
//! ring ([`o2rust::system::Emulator::take_audio_consumer`]). There is a
//! pacing mismatch: the emulator produces samples at the guest codec rate
//! per *emulated* second, and the host DAC wants e.g. 48k frames/s.
//!
//! Playback is anchored to the guest codec sample rate ([`start`] takes it
//! from the emulator), so the samples are resampled only when the host rate
//! differs from the guest rate — never to track the emulator's wall-clock
//! speed.  This keeps pitch/timbre accurate: consuming *faster* than the
//! guest rate (as the old production-rate tracker did when the interpreter
//! outran real time) shifted the boot chime up into a squealy high-frequency
//! tone, and chasing the bursty production rate caused it to alternate.
//!
//! Key robustness features:
//! - **Rate anchor**: the callback consumes at the guest codec rate, only
//!   drifting down when the backlog is low, so pitch never rises.
//! - **Minimum cushion** prevents premature drain when the ring fills
//!   during slow CPU emulation phases.
//! - **Underflow protection**: when the queue is low, consumption slows
//!   (audio stretches) rather than producing clicks.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, Stream, StreamConfig};
use rtrb::Consumer;

/// Owned audio sink; dropping it stops the stream (and the feeder thread
/// once the channel closes).
pub struct AudioOut {
    _stream: Stream,
}

/// Shared guest-sample FIFO awaiting playback.
struct Fifo {
    /// Guest frames not yet consumed by playback.
    queue: VecDeque<f32>,
}

impl Default for Fifo {
    fn default() -> Self {
        Self {
            queue: VecDeque::with_capacity(1 << 16),
        }
    }
}

/// Drain the guest ring into the shared FIFO until `shutdown` is signalled
/// (channel disconnect or flag set).
fn feeder_loop(mut consumer: Consumer<f32>, fifo: Arc<Mutex<Fifo>>) {
    loop {
        let mut n = 0usize;
        {
            let mut fifo = fifo.lock().unwrap();
            while n < 8192 {
                match consumer.pop() {
                    Ok(s) => {
                        fifo.queue.push_back(s);
                        n += 1;
                    }
                    Err(_) => break,
                }
            }
            // Cap memory usage if the host callback stalls for a long time.
            let cap = 1 << 18; // ~260k frames ≈ 5s at 50kHz
            let len = fifo.queue.len();
            if len > cap {
                fifo.queue.drain(..len - cap);
            }
        }
        if n == 0 {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

/// Start a cpal output stream that time-stretches guest audio running
/// through `consumer` onto the host device.
pub fn start(consumer: Consumer<f32>, guest_rate: f64) -> Result<AudioOut> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow!("no default audio output device"))?;

    let supported = device.default_output_config()?;
    let channels = supported.channels() as usize;
    let host_rate = supported.sample_rate() as f64;
    let sample_format = supported.sample_format();
    let config: StreamConfig = supported.into();

    tracing::info!(
        "audio stream: host sample_rate={}, channels={}, format={}, guest_rate={}",
        host_rate,
        channels,
        sample_format,
        guest_rate
    );

    let fifo = Arc::new(Mutex::new(Fifo::default()));
    std::thread::spawn({
        let fifo = Arc::clone(&fifo);
        move || feeder_loop(consumer, fifo)
    });

    let stream = match sample_format {
        SampleFormat::F32 => build::<f32>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::I16 => build::<i16>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::U16 => build::<u16>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::I8 => build::<i8>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::U8 => build::<u8>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::I24 => build::<i32>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::U24 => build::<u32>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::I32 => build::<i32>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::U32 => build::<u32>(&device, &config, channels, host_rate, guest_rate, fifo),
        SampleFormat::F64 => build::<f64>(&device, &config, channels, host_rate, guest_rate, fifo),
        other => return Err(anyhow!("unsupported audio sample format: {other}")),
    }?;

    stream.play()?;
    Ok(AudioOut { _stream: stream })
}

fn build<T>(
    device: &cpal::Device,
    config: &StreamConfig,
    channels: usize,
    host_rate: f64,
    guest_rate: f64,
    fifo: Arc<Mutex<Fifo>>,
) -> Result<Stream>
where
    T: SizedSample + FromSample<f32>,
{
    // Fractional read position between `prev` and the FIFO front.
    let mut frac = 0.0f64;
    let mut prev = 0.0f32;
    // Track silence duration to apply a gentle fade-out on long underruns.
    let mut silence_samples: u32 = 0;
    // Smooth the consumption rate across callbacks to avoid jitter.
    let mut smooth_rate = 0.0f64;

    let stream = device
        .build_output_stream(
            config.clone(),
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut fifo = fifo.lock().unwrap();

                let queue = &mut fifo.queue;
                let mut queue_len = queue.len();

                // The guest audio is authored at `guest_rate` (the emulated
                // codec's output rate).  Consuming faster than `guest_rate`
                // pitch-shifts it up; consuming slower stretches it down.
                // Anchor the rate at `guest_rate` and only drift *down* as
                // cushion management demands, so the boot chime keeps its
                // iconic pitch/timbre regardless of emulator wall-clock speed.
                // Minimum cushion: the production rate in frames per 50ms.
                // We must not drain below this threshold without slowing down
                // first, so that bursts have time to accumulate.
                let min_cushion = (guest_rate * 0.050).max(64.0);
                // Soft target: 150ms of backlog — comfortable room for bursts.
                let target = (guest_rate * 0.150).max(256.0);

                // Compute the desired consumption rate.
                // goal: consume at guest_rate while keeping queue near `target`.
                let excess = queue_len as f64 - target;
                let drift_ratio = if target > 0.0 {
                    excess / target
                } else {
                    0.0
                };
                // Drift correction: only nudge consumption *down* (±15%) to
                // keep the backlog from draining; never up, or pitch rises.
                let correction = (drift_ratio * 0.15).clamp(-0.25, 0.0);
                let mut rate = guest_rate * (1.0 + correction);
                rate = rate.clamp(guest_rate * 0.08, guest_rate);

                // When the queue is dangerously low, slow down dramatically
                // to stretch the remaining samples over a longer window
                // (avoids clicking from rapid pops then empty).
                if (queue_len as f64) < min_cushion && queue_len > 0 {
                    let ratio = queue_len as f64 / min_cushion;
                    // Drop rate smoothly down to guest_rate * 0.08 as queue
                    // approaches zero.
                    let floor = guest_rate * 0.08;
                    rate = floor + (rate - floor) * ratio;
                }

                // When the queue is empty, if we've already started playing,
                // decelerate to near-zero (stretch the last sample) rather
                // than clicking. Once the queue refills, resume at guest_rate.
                if queue_len == 0 && prev != 0.0 {
                    rate = 0.0;
                }

                // Smooth the rate across callbacks to avoid jitter from
                // per-callback queue snapshots.
                if rate > 0.0 {
                    smooth_rate = 0.3 * rate + 0.7 * smooth_rate;
                    if smooth_rate < 1.0 {
                        smooth_rate = rate;
                    }
                } else {
                    // Smooth toward zero, but don't hold indefinitely.
                    smooth_rate *= 0.95;
                }
                let step = smooth_rate / host_rate;

                for frame in data.chunks_mut(channels) {
                    let sample = if queue_len == 0 || smooth_rate < 0.5 {
                        // Underrun or fully slowed: hold last sample with a
                        // very gentle fade to silence over ~200ms.
                        silence_samples += 1;
                        let fade = if silence_samples < 10000 {
                            1.0 - (silence_samples as f32 / 10000.0)
                        } else {
                            0.0
                        };
                        prev *= fade;
                        frac = 0.0;
                        prev
                    } else {
                        silence_samples = 0;
                        // Advance the fractional read position; pop whole frames.
                        frac += step;
                        while frac >= 1.0 && queue_len > 0 {
                            if let Some(s) = queue.pop_front() {
                                prev = s;
                            }
                            frac -= 1.0;
                            queue_len -= 1;
                        }
                        let next = queue.front().copied().unwrap_or(prev);
                        prev + (next - prev) * frac as f32
                    };
                    let out = T::from_sample(sample.clamp(-1.0, 1.0));
                    for ch in frame.iter_mut() {
                        *ch = out;
                    }
                }
            },
            |err| {
                tracing::error!("audio stream error: {err}");
            },
            None,
        )
        .map_err(|e| anyhow!("failed to build output stream: {e}"))?;
    Ok(stream)
}
