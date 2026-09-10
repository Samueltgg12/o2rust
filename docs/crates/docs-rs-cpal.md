Low-level library for audio input and output, written in Rust.

For higher-level audio playback and capture, consider Rodio or similar libraries.
How to use cpal

Here are some concepts cpal exposes:

    A Host provides access to the available audio devices on the system. Some platforms have more than one host available, but every platform supported by CPAL has at least one default_host that is guaranteed to be available.
    A Device is an audio device that may have any number of input and output streams.
    A Stream is an open flow of audio data. Input streams allow you to receive audio data, output streams allow you to play audio data. You must choose which Device will run your stream before you can create one. Often, a default device can be retrieved via the Host.

The first step is to initialise the Host:

use cpal::traits::HostTrait;
let host = cpal::default_host();

Then choose an available Device. The easiest way is to use the default input or output Device via the default_input_device() or default_output_device() methods on host.

Alternatively, you can enumerate all the available devices with the devices() method. Beware that the default_*_device() functions return an Option<Device> in case no device is available for that stream type on the system.

let device = host.default_output_device().expect("no output device available");

Before we can create a stream, we must decide what the configuration of the audio stream is going to be. You can query all the supported configurations with the supported_input_configs() and supported_output_configs() methods. These produce a list of SupportedStreamConfigRange structs which can later be turned into actual SupportedStreamConfig structs.

If you don’t want to query the list of configs, you can also build your own StreamConfig manually, but doing so could lead to an error when building the stream if the config is not supported by the device.

    Note: the supported_input/output_configs() methods could return an error for example if the device has been disconnected.

use cpal::traits::{DeviceTrait, HostTrait};
let mut supported_configs_range = device.supported_output_configs()
    .expect("error while querying configs");
let supported_config = supported_configs_range.next()
    .expect("no supported config?!")
    .with_max_sample_rate();

Now that we have everything for the stream, we are ready to create it from our selected device:

use cpal::Data;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
let stream = device.build_output_stream(
    config,
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // react to stream events and read or write stream data here.
    },
    move |err| {
        // react to errors here.
    },
    None // Timeout for stream initialization: None = wait indefinitely,
         // Some(Duration) = time to wait for the backend to initialize the stream.
);

Streams are returned in a paused state. Once the stream has been started with Stream::play, the selected audio device will periodically call the data callback that was passed to the function. For input streams, the callback receives &Data containing captured audio samples. For output streams, the callback receives &mutData to be filled with audio samples for playback.

    Note: Creating and running a stream will not block the thread. On modern platforms, the given callback is called by a dedicated, high-priority thread responsible for delivering audio data to the system’s audio device in a timely manner. On older platforms that only provide a blocking API (e.g. ALSA), CPAL will create a thread in order to consistently provide non-blocking behaviour (currently this is a thread per stream, but this may change to use a single thread for all streams). If this is an issue for your platform or design, please share your issue and use-case with the CPAL team on the GitHub issue tracker for consideration.

In this example, we simply fill the given output buffer with silence.

use cpal::{Data, Sample, SampleFormat, FromSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
let sample_format = supported_config.sample_format();
let config = supported_config.into();
let stream = match sample_format {
    SampleFormat::F32 => device.build_output_stream(config, write_silence::<f32>, err_fn, None),
    SampleFormat::I16 => device.build_output_stream(config, write_silence::<i16>, err_fn, None),
    SampleFormat::U16 => device.build_output_stream(config, write_silence::<u16>, err_fn, None),
    sample_format => panic!("Unsupported sample format '{sample_format}'")
}.unwrap();

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}

Streams are always returned in a paused state, so we must call Stream::play to start running the data callback.

stream.play().unwrap();

Some devices support pausing the audio stream. This can be useful for saving energy in moments of silence.

stream.pause().unwrap();

Re-exports

pub use device_description::DeviceDescription;
pub use device_description::DeviceDescriptionBuilder;
pub use device_description::DeviceDirection;
pub use device_description::DeviceType;
pub use device_description::InterfaceType;
pub use platform::available_hosts;
pub use platform::default_host;
pub use platform::host_from_id;
pub use platform::Device;
pub use platform::Devices;
pub use platform::Host;
pub use platform::HostId;
pub use platform::Stream;
pub use platform::SupportedInputConfigs;
pub use platform::SupportedOutputConfigs;
pub use platform::ALL_HOSTS;

Modules

device_description
    Device metadata and description types.
platform
    Platform-specific items.
traits
    The suite of traits allowing CPAL to abstract over hosts, devices, event loops and stream IDs.

Macros

assert_stream_send
    Compile-time assertion that a stream type implements Send.
assert_stream_sync
    Compile-time assertion that a stream type implements Sync.

Structs

Data
    A buffer of dynamically typed audio data, passed to raw stream callbacks.
DeviceId
    A stable identifier for an audio device across all supported platforms.
Error
    Error type for all CPAL operations.
I24
    24-bit signed integer sample type.
InputCallbackInfo
    Information relevant to a single call to the user’s input stream data callback.
InputStreamTimestamp
    A timestamp associated with a call to an input stream’s data callback.
OutputCallbackInfo
    Information relevant to a single call to the user’s output stream data callback.
OutputStreamTimestamp
    A timestamp associated with a call to an output stream’s data callback.
StreamConfig
    The set of parameters used to describe how to open a stream.
StreamInstant
    A monotonic time instance associated with a stream, retrieved from either:
SupportedStreamConfig
    Describes a single supported stream configuration, retrieved via either a SupportedStreamConfigRange instance or one of the Device::default_input/output_config methods.
SupportedStreamConfigRange
    Describes a range of supported stream configurations, retrieved via the Device::supported_input/output_configs method.
U24
    24-bit unsigned integer sample type.

Enums

BufferSize
    The buffer size requests the callback size for audio streams.
ErrorKind
    A list specifying general categories of CPAL error.
SampleFormat
    Format that each sample has. Usually, this corresponds to the sampling depth of the audio source. For example, 16 bit quantized samples can be encoded in i16 or u16. Note that the quantized sampling depth is not directly visible for formats where is_float is true.
SupportedBufferSize
    Describes the minimum and maximum supported buffer size for the device

Constants

SAMPLE_RATE_48K
    48 kHz: the EBU/SMPTE broadcast standard, default on most modern hardware.
SAMPLE_RATE_CD
    44.1 kHz: the CD standard, widely used in music production and consumer audio.

Traits

FromSample
    Similar to the std From trait, but specifically for converting between sample types.
Sample
    A trait for working generically across different Sample format types.
SizedSample
    A Sample type with a known corresponding SampleFormat.

Type Aliases

ChannelCount
    Number of channels.
DevicesFiltered
    Iterator of devices wrapped in a filter to only include certain device types
FrameCount
    A frame represents one sample for each channel. For example, with stereo audio, one frame contains two samples (left and right channels).
InputDevices
    A host’s device iterator yielding only input devices.
OutputDevices
    A host’s device iterator yielding only output devices.
SampleRate
    The number of samples processed per second for a single channel of audio.