//! Audio output: cpal stream fed by the emulator's rtrb ring.
//!
//! The core MACE audio state pushes mono `f32` frames into an [`rtrb`]
//! ring buffer ([`o2rust::system::Emulator::take_audio_consumer`]); this
//! module drains it on a cpal callback thread and scales to the host's sample
//! format / channel count.

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, Stream, StreamConfig};
use rtrb::Consumer;

/// Owned audio sink; dropping it stops the stream.
pub struct AudioOut {
    _stream: Stream,
}

/// Start a cpal output stream that drains `consumer`.
pub fn start(consumer: Consumer<f32>) -> Result<AudioOut> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow!("no default audio output device"))?;

    let supported = device.default_output_config()?;
    let channels = supported.channels() as usize;
    let sample_rate = supported.sample_rate();
    let sample_format = supported.sample_format();
    let config: StreamConfig = supported.into();

    tracing::debug!(
        "audio stream: sample_rate={}, channels={}, format={}",
        sample_rate,
        channels,
        sample_format
    );

    let stream = match sample_format {
        SampleFormat::F32 => build::<f32>(&device, &config, channels, consumer),
        SampleFormat::I16 => build::<i16>(&device, &config, channels, consumer),
        SampleFormat::U16 => build::<u16>(&device, &config, channels, consumer),
        SampleFormat::I8 => build::<i8>(&device, &config, channels, consumer),
        SampleFormat::U8 => build::<u8>(&device, &config, channels, consumer),
        SampleFormat::I24 => build::<i32>(&device, &config, channels, consumer),
        SampleFormat::U24 => build::<u32>(&device, &config, channels, consumer),
        SampleFormat::I32 => build::<i32>(&device, &config, channels, consumer),
        SampleFormat::U32 => build::<u32>(&device, &config, channels, consumer),
        SampleFormat::F64 => build::<f64>(&device, &config, channels, consumer),
        other => return Err(anyhow!("unsupported audio sample format: {other}")),
    }?;

    stream.play()?;
    Ok(AudioOut { _stream: stream })
}

fn build<T>(
    device: &cpal::Device,
    config: &StreamConfig,
    channels: usize,
    mut consumer: Consumer<f32>,
) -> Result<Stream>
where
    T: SizedSample + FromSample<f32>,
{
    let stream = device
        .build_output_stream(
            config.clone(),
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                // One emulated (mono) frame per output frame; duplicate it
                // across all host channels while it lasts, else silence.
                for frame in data.chunks_mut(channels) {
                    let sample = consumer
                        .pop()
                        .map(|v| v.clamp(-1.0, 1.0))
                        .unwrap_or(0.0);
                    let sample = T::from_sample(sample);
                    for out in frame.iter_mut() {
                        *out = sample;
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