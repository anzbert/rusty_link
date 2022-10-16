use crate::constants::*;
use crate::synth::MonoSine;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BufferSize, Device, Host, OutputCallbackInfo, Sample, SampleFormat, StreamInstant,
    SupportedStreamConfig,
};
use cpal::{Stream, StreamConfig};
use std::time;

pub struct AudioPlatformCpal {
    host: Host,
    device: Device,
    pub config: StreamConfig,
    supported_config: SupportedStreamConfig,
    output_latency: u64,
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

        let supported_config = supported_configs_range
            .next()
            .expect("No supported config?!")
            .with_max_sample_rate();

        let mut config = supported_config.config();
        config.buffer_size = BufferSize::Fixed(511);

        Self {
            output_latency: 0,
            host,
            device,
            config,
            supported_config,
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
        mut engine_callback: (impl FnMut() + Send + 'static),
    ) -> impl FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static {
        let mut sine = MonoSine::new(config.sample_rate.0, LOW_TONE);
        let data_fn = move |data: &mut [T], info: &cpal::OutputCallbackInfo| {
            let output_latency = info
                .timestamp()
                .playback
                .duration_since(&info.timestamp().callback)
                .unwrap_or_default(); // Default is 0

            engine_callback();

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

pub fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}
