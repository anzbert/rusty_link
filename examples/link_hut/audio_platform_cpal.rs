use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, OutputCallbackInfo, Sample, SampleFormat, SupportedStreamConfig};
use cpal::{Stream, StreamConfig};

const BUFFER_SIZE: u32 = 512;

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
            device.name().expect("Could not get device name. "),
        );

        println!(
            "BUFFER SIZE: {} samples, {:.2} ms (Buffer Size {:?})",
            BUFFER_SIZE,
            BUFFER_SIZE as f64 * 1000. / config.sample_rate.0 as f64,
            first_supported_config.buffer_size()
        );

        let channel_cfg = match config.channels {
            1 => " (Mono)",
            2 => " (Stereo)",
            _ => "",
        };
        println!("OUTPUT CHANNELS: {}{}", config.channels, channel_cfg);

        // println!("OUTPUT DEVICE LATENCY: {}", todo!()); // not available in cpal

        Self {
            device,
            config,
            supported_config: first_supported_config,
        }
    }

    fn build_cpal_callback<T: Sample>(
        &self,
        mut engine_callback: (impl FnMut(usize, u64, f64, u32) -> Vec<f32> + Send + 'static),
    ) -> impl FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static {
        let config_clone = self.config.clone();

        let data_fn = move |data: &mut [T], info: &cpal::OutputCallbackInfo| {
            // output latency in micros
            let output_latency = info
                .timestamp()
                .playback
                .duration_since(&info.timestamp().callback)
                .unwrap_or_default()
                .as_micros() as u64;

            // size of output buffer in samples
            let buffer_size: usize = data.len() / config_clone.channels as usize;

            // sample time in micros per sample at the current sample rate
            let sample_time_micros: f64 = 1_000_000. / config_clone.sample_rate.0 as f64;

            // invoke callback which builds a latency compensated buffer and handles link audio sessionstate
            let buffer: Vec<f32> = engine_callback(
                buffer_size,
                output_latency,
                sample_time_micros,
                config_clone.sample_rate.0,
            );

            // send buffer with same sound on all channels (equals mono output) to output
            for s in 0..data.len() / config_clone.channels as usize {
                for c in 0..config_clone.channels as usize {
                    data[s * config_clone.channels as usize + c] = Sample::from(&buffer[s]);
                }
            }
        };
        return data_fn;
    }

    pub fn build_stream<T: Sample>(
        &self,
        engine_callback: (impl FnMut(usize, u64, f64, u32) -> Vec<f32> + Send + 'static),
    ) -> Stream {
        let callback = self.build_cpal_callback::<f32>(engine_callback);

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let stream = match self.supported_config.sample_format() {
            SampleFormat::F32 => self
                .device
                .build_output_stream(&self.config, callback, err_fn),
            SampleFormat::I16 => self
                .device
                .build_output_stream(&self.config, callback, err_fn),
            SampleFormat::U16 => self
                .device
                .build_output_stream(&self.config, callback, err_fn),
        }
        .unwrap();

        stream.play().unwrap();

        stream
    }
}
