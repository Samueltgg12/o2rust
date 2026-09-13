//! Audio output: cpal stream fed by the emulator's rtrb ring.
//!
//! The core MACE audio pushes mono guest-rate `f32` frames into an [`rtrb`]
//! ring ([`o2rust::system::Emulator::take_audio_consumer`]). There is a
//! fundamental mismatch: the emulator produces samples at the guest codec
//! rate per *emulated* second, but the CPU interpreter runs far below
//! real time, so in wall-clock time the guest only generates on the order
//! of ~1k frames/s while the host DAC wants e.g. 48k frames/s. Playing the
//! ring 1:1 therefore underruns constantly — audible as crackling/popping.
//!
//! This module time-stretches the guest audio over the wall-clock time the
//! guest spends generating it (what a *listener* perceives as correct
//! pacing: the boot chime lasts exactly as long as the emulated machine
//! spends playing it). A feeder thread drains the rtrb ring into a shared
//! FIFO and continuously estimates the guest's real-time production rate;
//! the cpal callback consumes from the FIFO at that rate with linear
//! interpolation, so playback is smooth regardless of emulator speed.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, Stream, StreamConfig};
use rtrb::Consumer;

/// Owned audio sink; dropping it stops the stream (and the feeder thread
/// once the channel closes).
pub struct AudioOut {
    _stream: Stream,
}

/// Shared guest-sample FIFO plus the running production-rate estimate.
struct Fifo {
    /// Guest frames not yet consumed by playback.
    queue: VecDeque<f32>,
    /// Estimated guest production rate in frames per wall-clock second.
    prod_rate: f64,
    /// Last time the feeder pushed frames.
    last_push: Option<Instant>,
    /// Frames pushed since `last_push`.
    pushed: u64,
}

impl Default for Fifo {
    fn default() -> Self {
        Self {
            queue: VecDeque::with_capacity(1 << 16),
            // A plausible starting point; the EWMA converges quickly.
            prod_rate: 1000.0,
            last_push: None,
            pushed: 0,
        }
    }
}

impl Fifo {
    /// Register `n` frames arriving from the guest, updating the EWMA rate.
    fn note_push(&mut self, n: usize) {
        let now = Instant::now();
        if let Some(last) = self.last_push {
            let dt = now.duration_since(last).as_secs_f64();
            if dt > 1e-3 {
                let observed = self.pushed as f64 / dt;
                // EWMA; fast enough to track bursts, slow enough to be stable.
                self.prod_rate = 0.3 * observed + 0.7 * self.prod_rate;
                self.last_push = Some(now);
                self.pushed = 0;
            }
        } else {
            self.last_push = Some(now);
        }
        self.pushed += n as u64;
    }
}

/// Drain the guest ring into the shared FIFO until `shutdown` is signalled
/// (channel disconnect or flag set).
fn feeder_loop(mut consumer: Consumer<f32>, fifo: Arc<Mutex<Fifo>>) {
    loop {
        let mut n = 0usize;
        {
            let mut fifo = fifo.lock().unwrap();
            while n < 4096 {
                match consumer.pop() {
                    Ok(s) => {
                        fifo.queue.push_back(s);
                        n += 1;
                    }
                    Err(_) => break,
                }
            }
            if n > 0 {
                fifo.note_push(n);
            }
            // Cap memory usage if the host callback stalls for a long time.
            let len = fifo.queue.len();
            fifo.queue.drain(..len.saturating_sub(1 << 18));
        }
        if n == 0 {
            std::thread::sleep(Duration::from_millis(2));
        }
    }
}

/// Start a cpal output stream that time-stretches guest audio running
/// through `consumer` onto the host device.
pub fn start(consumer: Consumer<f32>) -> Result<AudioOut> {
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
        "audio stream: host sample_rate={}, channels={}, format={}",
        host_rate,
        channels,
        sample_format
    );

    let fifo = Arc::new(Mutex::new(Fifo::default()));
    std::thread::spawn({
        let fifo = Arc::clone(&fifo);
        move || feeder_loop(consumer, fifo)
    });

    let stream = match sample_format {
        SampleFormat::F32 => build::<f32>(&device, &config, channels, host_rate, fifo),
        SampleFormat::I16 => build::<i16>(&device, &config, channels, host_rate, fifo),
        SampleFormat::U16 => build::<u16>(&device, &config, channels, host_rate, fifo),
        SampleFormat::I8 => build::<i8>(&device, &config, channels, host_rate, fifo),
        SampleFormat::U8 => build::<u8>(&device, &config, channels, host_rate, fifo),
        SampleFormat::I24 => build::<i32>(&device, &config, channels, host_rate, fifo),
        SampleFormat::U24 => build::<u32>(&device, &config, channels, host_rate, fifo),
        SampleFormat::I32 => build::<i32>(&device, &config, channels, host_rate, fifo),
        SampleFormat::U32 => build::<u32>(&device, &config, channels, host_rate, fifo),
        SampleFormat::F64 => build::<f64>(&device, &config, channels, host_rate, fifo),
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
    fifo: Arc<Mutex<Fifo>>,
) -> Result<Stream>
where
    T: SizedSample + FromSample<f32>,
{
    // Fractional read position between `prev` and the FIFO front.
    let mut frac = 0.0f64;
    let mut prev = 0.0f32;
    let stream = device
        .build_output_stream(
            config.clone(),
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut fifo = fifo.lock().unwrap();

                // Consumption rate: the guest's production rate (so playback
                // lasts as long as the guest spends audioing), with a small
                // drift correction toward keeping ~a 20 ms cushion so
                // bursty production doesn't dry us out between callbacks.
                let cushion_target = (fifo.prod_rate * 0.020).max(64.0);
                let drift = (fifo.queue.len() as f64 - cushion_target) / cushion_target;
                let rate = (fifo.prod_rate * (1.0 + 0.05 * drift.clamp(-0.5, 0.5)))
                    .clamp(50.0, 200_000.0);
                let step = rate / host_rate;

                for frame in data.chunks_mut(channels) {
                    let sample = if fifo.queue.is_empty() {
                        // Underrun: decay the tail smoothly to silence rather
                        // than holding a DC level (which would click/thump).
                        prev *= 0.999;
                        frac = 0.0;
                        prev
                    } else {
                        // Advance the fractional read position; pop whole frames.
                        frac += step;
                        while frac >= 1.0 {
                            prev = fifo.queue.pop_front().unwrap_or(prev);
                            frac -= 1.0;
                        }
                        let next = fifo.queue.front().copied().unwrap_or(prev);
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
