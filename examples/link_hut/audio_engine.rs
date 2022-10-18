use crate::{audio_platform_cpal::AudioPlatformCpal, InputCommand};
use cpal::Stream;
use rusty_link::{AblLink, SessionState};
use std::{f32::consts::TAU, sync::mpsc::Receiver};

// https://pages.mtu.edu/~suits/notefreqs.html
const HIGH_TONE: f32 = 1567.98; // G6
const LOW_TONE: f32 = 1108.73; // C#6

pub const CLICK_DURATION: i64 = 100_000; // 100 ms click duration in micros

pub struct AudioEngine {
    input: Receiver<InputCommand>,
    pub link: &'static AblLink,
    engine_data: EngineData,
    pub quantum: f64,
    audio_platorm: AudioPlatformCpal,
    stream: Stream,
}

impl AudioEngine {
    pub fn new(
        link: &'static AblLink,
        audio_cpal: AudioPlatformCpal,
        input: Receiver<InputCommand>,
    ) -> Self {
        let mut time_at_last_click = 0.;
        let mut synth_clock: u32 = 0;
        let mut audio_session_state = SessionState::new();

        let engine_callback = move |buffer_size: usize,
                                    output_latency: i64,
                                    sample_time_micros: f64,
                                    sample_rate: u32| {
            // ---- handle audio session state ----
            link.capture_audio_session_state(&mut audio_session_state);
            // let engine_data = pull_engine_data();

            let quantum = 4.; // !!! temp value REMOVE !!!

            // ----  build a latency compensated buffer ----

            let mut buffer: Vec<f32> = Vec::with_capacity(buffer_size);

            let begin_time = link.clock_micros() + output_latency;

            for i in 0..buffer_size {
                let mut y_amplitude: f32 = 0.;

                // Compute the host time for this sample and the last.
                let host_time = begin_time as f64 + (sample_time_micros * i as f64);
                let last_sample_host_time = host_time as f64 - sample_time_micros;

                // Only make sound for positive beat magnitudes. Negative beat
                // magnitudes are count-in beats.
                if audio_session_state.beat_at_time(host_time as i64, quantum) >= 0. {
                    // If the phase wraps around between the last sample and the
                    // current one with respect to a 1 beat quantum, then a click
                    // should occur.
                    if audio_session_state.phase_at_time(host_time as i64, 1.0)
                        < audio_session_state.phase_at_time(last_sample_host_time as i64, 1.0)
                    {
                        time_at_last_click = host_time;
                        synth_clock = 0; // reset synth clock
                    }

                    let micro_seconds_after_click = host_time - time_at_last_click;

                    // If we're within the click duration of the last beat, render
                    // the click tone into this sample
                    if micro_seconds_after_click < CLICK_DURATION as f64 {
                        // If the phase of the last beat with respect to the current
                        // quantum was zero, then it was at a quantum boundary and we
                        // want to use the high tone. For other beats within the
                        // quantum, use the low tone.

                        let freq = match audio_session_state
                            .phase_at_time(host_time as i64, quantum)
                            .floor() as usize
                        {
                            0 => HIGH_TONE,
                            _ => LOW_TONE,
                        };

                        let x_time = synth_clock as f32 / sample_rate as f32;

                        // Simple cosine synth:
                        y_amplitude =
                            (TAU * x_time * freq).cos() * (1. - (2.5 * TAU * x_time).sin());

                        // For testing, sine synth:
                        // y_amplitude = (x_time * freq * TAU).sin();

                        synth_clock = (synth_clock + 1) % sample_rate;
                    }
                }
                buffer.push(y_amplitude);

                // For testing, constant sine synth:
                // buffer.push(((synth_clock as f32 / 48000.) as f32 * 440. * TAU).sin());
            }

            buffer
        };

        let stream_callback =
            AudioPlatformCpal::build_callback::<f32>(audio_cpal.config.clone(), engine_callback);

        let stream = audio_cpal.build_stream(stream_callback);

        Self {
            link,
            input,
            engine_data: EngineData { quantum: 4. },
            quantum: 4.,
            audio_platorm: audio_cpal,
            stream,
        }
    }

    // fn now(&self) -> i64 {
    //     self.link.clock_micros()
    // }

    // pub fn start_playing(&mut self) {
    //     let mut session_state = SessionState::new();
    //     self.link.capture_app_session_state(&mut session_state);
    //     session_state.set_is_playing_and_request_beat_at_time(
    //         true,
    //         self.now() as u64,
    //         0.,
    //         self.quantum,
    //     );
    //     self.link.commit_app_session_state(&session_state);
    // }

    // pub fn stop_playing(&mut self) {
    //     let mut session_state = SessionState::new();
    //     self.link.capture_app_session_state(&mut session_state);
    //     session_state.set_is_playing(true, self.now() as u64);
    //     self.link.commit_app_session_state(&session_state);
    // }

    // pub fn is_playing(&self) -> bool {
    //     let mut session_state = SessionState::new();
    //     self.link.capture_app_session_state(&mut session_state);
    //     session_state.is_playing()
    // }

    // pub fn beat_time(&self) -> f64 {
    //     let mut session_state = SessionState::new();
    //     self.link.capture_app_session_state(&mut session_state);
    //     session_state.beat_at_time(self.now(), self.quantum)
    // }

    // pub fn set_tempo(&mut self, tempo: f64) {
    //     let mut session_state = SessionState::new();
    //     self.link.capture_app_session_state(&mut session_state);
    //     session_state.set_tempo(tempo, self.now());
    //     self.link.commit_app_session_state(&session_state);
    // }

    // pub fn quantum(&self) -> f64 {
    //     self.quantum
    // }

    // pub fn set_quantum(&mut self, quantum: f64) {
    //     self.quantum = quantum;
    // }

    // pub fn is_start_stop_sync_enabled(&self) -> bool {
    //     self.link.is_start_stop_sync_enabled()
    // }

    // pub fn set_start_stop_sync_enabled(&mut self, enabled: bool) {
    //     self.link.enable_start_stop_sync(enabled);
    // }
}

struct EngineData {
    quantum: f64,
}
