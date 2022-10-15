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
        let buffer_size = 255;
        config.buffer_size = BufferSize::Fixed(buffer_size);

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        let mut sine = MonoSine::new(config.sample_rate.0 as f32, config.channels, 440.0);

        let data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let buffer = sine.progress(buffer_size);

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

pub struct MonoSine {
    sample_clock: f32,
    sample_rate: f32,
    channels: u16,
    pitch_freq: f32,
}

impl MonoSine {
    pub fn new(sample_rate: f32, channels: u16, pitch_freq: f32) -> Self {
        Self {
            sample_clock: 0.,
            sample_rate,
            channels,
            pitch_freq,
        }
    }

    pub fn progress(&mut self, buffer_size: u32) -> Vec<f32> {
        let mut buffer = Vec::<f32>::with_capacity((buffer_size * self.channels as u32) as usize);

        for _ in 0..buffer_size {
            self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;

            let sample = (self.sample_clock * self.pitch_freq * 2.0 * std::f32::consts::PI
                / self.sample_rate)
                .sin();

            for _channel in 0..self.channels {
                buffer.push(sample);
            }
        }

        buffer
    }
}
