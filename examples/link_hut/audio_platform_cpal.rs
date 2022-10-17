use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, Device, OutputCallbackInfo, Sample, SampleFormat, SupportedStreamConfig};
use cpal::{Stream, StreamConfig};

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
        config.buffer_size = BufferSize::Fixed(511);
        // config.channels = 2;

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

        // stream.pause().unwrap();
        stream
    }

    // not sure if this should be here?!
    pub fn build_callback<T: Sample>(
        config: StreamConfig,
        mut engine_callback: (impl FnMut(u64, u64) -> Vec<f32> + Send + 'static),
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
            let buffer_size: u64 = data.len() as u64 / config.channels as u64;

            // sample time in micros per sample at the current sample rate
            let sample_time_micros: f64 = 1_000_000. / config.sample_rate.0 as f64;

            // output latency in samples
            let latency_in_samples = (output_latency as f64 / sample_time_micros) as u64;

            // invoke callback which builds a latency compensated buffer and handles link audio sessionstate
            let buffer: Vec<f32> = engine_callback(buffer_size, latency_in_samples);

            // send buffer with same sound on all channels (equals mono output) to output
            for s in 0..data.len() / config.channels as usize {
                for c in 0..config.channels as usize {
                    data[s * 2 + c] = Sample::from(&buffer[s]);
                }
            }
        };
        return data_fn;
    }
}

// pub fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
//     for sample in data.iter_mut() {
//         *sample = Sample::from(&0.0);
//     }
// }
