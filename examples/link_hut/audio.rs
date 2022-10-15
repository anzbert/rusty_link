use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, Device, Host, Sample, SampleFormat, SupportedStreamConfig};
use cpal::{Stream, StreamConfig};

pub struct AudioEngine {
    pub host: Host,
    pub device: Device,
    pub config: StreamConfig,
    pub supported_config: SupportedStreamConfig,
    pub stream: Stream,
}

impl AudioEngine {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");

        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let mut config = supported_config.config();
        config.buffer_size = BufferSize::Fixed(255);

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        let sample_rate = config.sample_rate.0 as f32; // something like 44100
        let mut sample_clock = 0f32;

        let data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let buffer_size = data.len();
            let mut buffer = Vec::<f32>::with_capacity(buffer_size);

            for _sample in 0..buffer_size / 2 {
                sample_clock = (sample_clock + 1.0) % sample_rate;
                // println!("{}", sample_clock);
                buffer
                    .push((sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin());
                buffer
                    .push((sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin());
            }

            for (i, sample) in data.iter_mut().enumerate() {
                *sample = Sample::from(&buffer[i]);
            }
        };

        let sample_format = supported_config.sample_format();

        let stream = match sample_format {
            SampleFormat::F32 => device.build_output_stream(&config, data_fn, err_fn),
            SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
            SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
        }
        .unwrap();

        Self {
            host,
            device,
            config,
            supported_config,
            stream,
        }
    }
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}
