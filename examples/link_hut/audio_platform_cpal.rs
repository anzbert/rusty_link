use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, Device, OutputCallbackInfo, Sample, SampleFormat, SupportedStreamConfig};
use cpal::{Stream, StreamConfig};

const BUFFER_SIZE: u32 = 512;

pub struct AudioPlatformCpal {
    device: Device,
    pub config: StreamConfig,
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
        // config.channels = 1;

        println!("SAMPLE RATE: {}", config.sample_rate.0);

        println!(
            "DEVICE NAME: {}",
            device.name().expect("Could not get device name. "),
        );

        println!(
            "BUFFER SIZE: {} samples, {:.2} ms",
            BUFFER_SIZE,
            BUFFER_SIZE as f64 * 1000. / config.sample_rate.0 as f64
        );

        let channel_cfg = match config.channels {
            1 => " (Mono)",
            2 => " (Stereo)",
            _ => "",
        };
        println!("OUTPUT CHANNELS: {}{}", config.channels, channel_cfg);

        // println!("OUTPUT DEVICE LATENCY: {}", todo!());

        Self {
            device,
            config,
            supported_config: first_supported_config,
        }
    }

    pub fn build_stream<T: Sample>(
        &self,
        callback: impl FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static,
    ) -> Stream {
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

        // stream.play().unwrap();

        stream
    }

    pub fn build_callback<T: Sample>(
        config: StreamConfig,
        mut engine_callback: (impl FnMut(usize, i64, f64) -> Vec<f32> + Send + 'static),
    ) -> impl FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static {
        let data_fn = move |data: &mut [T], info: &cpal::OutputCallbackInfo| {
            // output latency in micros
            let output_latency = info
                .timestamp()
                .playback
                .duration_since(&info.timestamp().callback)
                .unwrap_or_default()
                .as_micros();

            // size of output buffer in samples
            let buffer_size: usize = data.len() / config.channels as usize;

            // sample time in micros per sample at the current sample rate
            let sample_time_micros: f64 = 1_000_000. / config.sample_rate.0 as f64;

            // invoke callback which builds a latency compensated buffer and handles link audio sessionstate
            let buffer: Vec<f32> =
                engine_callback(buffer_size, output_latency as i64, sample_time_micros);

            // send buffer with same sound on all channels (equals mono output) to output
            for s in 0..data.len() / config.channels as usize {
                for c in 0..config.channels as usize {
                    data[s * config.channels as usize + c] = Sample::from(&buffer[s]);
                }
            }

            // for 1 channel test:
            // for s in 0..data.len() {
            //     data[s] = Sample::from(&buffer[s]);
            // }
        };
        return data_fn;
    }
}
