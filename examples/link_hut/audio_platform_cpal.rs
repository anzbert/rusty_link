use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BufferSize, Device, FromSample, OutputCallbackInfo, Sample, SampleFormat, SupportedStreamConfig,
};
use cpal::{Stream, StreamConfig};
use std::time::Duration;

/// Audio Buffer size
const BUFFER_SIZE: u32 = 512;

/// Handles Multiplatform audio output with 'cpal'.
pub struct AudioPlatformCpal {
    config: StreamConfig,
    device: Device,
    supported_config: SupportedStreamConfig,
}

impl AudioPlatformCpal {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("No output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("Error while querying configs");

        let first_supported_config = supported_configs_range
            .next()
            .expect("No supported config?!")
            .with_max_sample_rate();

        let mut config = first_supported_config.config();
        config.buffer_size = BufferSize::Fixed(BUFFER_SIZE);

        println!(
            "SAMPLE RATE: {} (SampleFormat::{:?})",
            config.sample_rate.0,
            first_supported_config.sample_format()
        );

        println!(
            "DEVICE NAME: {}",
            device.name().expect("Could not get device name."),
        );

        println!(
            "BUFFER SIZE: {} samples, {:.2} ms (Supported {:?})",
            BUFFER_SIZE,
            BUFFER_SIZE as f64 * 1000. / config.sample_rate.0 as f64,
            first_supported_config.buffer_size()
        );

        let channel_cfg = match config.channels {
            1 => "(Mono)",
            2 => "(Stereo)",
            _ => "",
        };
        println!("OUTPUT CHANNELS: {} {}", config.channels, channel_cfg);

        Self {
            device,
            config,
            supported_config: first_supported_config,
        }
    }

    /// Build an Audio Stream in the correct format with a provided engine callback function
    pub fn build_stream<T: Sample>(
        &self,
        engine_callback: impl FnMut(usize, u64, Duration, Duration, u64) -> Vec<f32> + Send + 'static,
    ) -> Stream {
        let callback = self.build_cpal_callback::<f32>(engine_callback);

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let stream = match self.supported_config.sample_format() {
            SampleFormat::F32 => {
                self.device
                    .build_output_stream(&self.config, callback, err_fn, None)
            }
            SampleFormat::I16 => {
                self.device
                    .build_output_stream(&self.config, callback, err_fn, None)
            }
            SampleFormat::U16 => {
                self.device
                    .build_output_stream(&self.config, callback, err_fn, None)
            }
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        stream.play().unwrap();

        stream
    }

    /// Build an audio callback that can be used with cpal's [build_output_stream]
    fn build_cpal_callback<T: Sample + FromSample<f32>>(
        &self,
        mut engine_callback: impl FnMut(usize, u64, Duration, Duration, u64) -> Vec<f32>
        + Send
        + 'static,
    ) -> impl FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static {
        let config_clone = self.config.clone();

        // Total number of samples since stream creation, used as a clock that counts in samples
        let mut sample_count: u64 = 0;

        // Time per sample at the current sample rate
        let sample_time = Duration::from_secs(1).div_f64(self.config.sample_rate.0 as f64);

        move |data: &mut [T], info: &cpal::OutputCallbackInfo| {
            // Output latency (as predicted by cpal)
            let output_latency = info
                .timestamp()
                .playback
                .duration_since(&info.timestamp().callback)
                .unwrap_or_default();

            // Size of provided output buffer for one channel in samples
            let buffer_size: usize = data.len() / config_clone.channels as usize;

            // Invoke AudioEngine callback which builds a buffer of metronome clicks
            // and handles changes in the SessionState
            let buffer: Vec<f32> = engine_callback(
                buffer_size,
                config_clone.sample_rate.0 as u64,
                output_latency,
                sample_time,
                sample_count,
            );

            // Send buffer with same sound output to all channels (equals mono)
            for s in 0..data.len() / config_clone.channels as usize {
                for c in 0..config_clone.channels as usize {
                    // - Silence:
                    // data[s * config_clone.channels as usize + c] = Sample::EQUILIBRIUM;

                    // - Metronome:
                    data[s * config_clone.channels as usize + c] = T::from_sample(buffer[s]);
                }
            }

            // Increase sample counter clock
            sample_count += buffer_size as u64;
        }
    }
}
