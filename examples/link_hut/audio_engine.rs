use std::time;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BufferSize, Device, Host, OutputCallbackInfo, Sample, SampleFormat, SupportedStreamConfig,
};
use cpal::{Stream, StreamConfig};

use crate::constants::*;
use crate::synth::MonoSine;

pub struct AudioEngine {
    host: Host,
    device: Device,
    config: StreamConfig,
    pub stream: Stream,
}

impl AudioEngine {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("No output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("Error while querying configs");

        let supported_config = supported_configs_range
            .next()
            .expect("No supported config?!")
            .with_max_sample_rate();

        let config = supported_config.config();
        // config.buffer_size = BufferSize::Fixed(255);
        // println!("{:?}", config);

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let stream = match supported_config.sample_format() {
            SampleFormat::F32 => {
                device.build_output_stream(&config, Self::build::<f32>(config.clone()), err_fn)
            }
            SampleFormat::I16 => {
                device.build_output_stream(&config, Self::build::<i16>(config.clone()), err_fn)
            }
            SampleFormat::U16 => {
                device.build_output_stream(&config, Self::build::<u16>(config.clone()), err_fn)
            }
        }
        .unwrap();

        stream.pause().unwrap();

        Self {
            host,
            device,
            config,
            stream,
        }
    }

    pub fn click(&self) {
        self.stream.play().unwrap();
        std::thread::sleep(time::Duration::from_millis(CLICK_DURATION));
        self.stream.pause().unwrap();
    }

    fn build<T: Sample>(config: StreamConfig) -> impl FnMut(&mut [T], &OutputCallbackInfo) + Send {
        let mut sine = MonoSine::new(config.sample_rate.0, LOW_TONE);
        let data_fn = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for s in 0..data.len() {
                let next_sample = sine.next().unwrap();
                for _ in 0..config.channels {
                    data[s] = Sample::from(&next_sample);
                }
            }
        };
        return data_fn;
    }
}
