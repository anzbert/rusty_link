use crate::{audio_platform_cpal::AudioPlatformCpal, UpdateSessionState};
use cpal::Stream;
use rusty_link::{AblLink, SessionState};
use std::{
    f32::consts::TAU,
    sync::{mpsc::Receiver, Arc, Mutex},
};

// https://pages.mtu.edu/~suits/notefreqs.html
const HIGH_TONE: f32 = 1567.98; // G6
const LOW_TONE: f32 = 1108.73; // C#6
const CLICK_DURATION: i64 = 100_000; // Click duration in micros

pub struct AudioEngine {
    pub stream: Stream,
}

impl AudioEngine {
    pub fn new(
        link: &'static AblLink,
        audio_cpal: AudioPlatformCpal,
        input: Receiver<UpdateSessionState>,
        quantum: Arc<Mutex<f64>>,
    ) -> Self {
        // Working variables for callback:
        let mut time_at_last_click = 0;
        let mut synth_clock: u32 = 0;
        let mut audio_session_state = SessionState::new();
        let mut last_known_quantum = *quantum.lock().unwrap();

        // Build callback:
        let engine_callback = move |buffer_size: usize,
                                    output_latency: u64,
                                    sample_time_micros: f64,
                                    sample_rate: u32| {
            // ---- HANDLE AUDIO SESSION STATE ----
            link.capture_audio_session_state(&mut audio_session_state);

            if let Ok(q) = quantum.try_lock() {
                last_known_quantum = *q;
            };

            if let Ok(command) = input.try_recv() {
                match command {
                    UpdateSessionState::TempoPlus => {
                        audio_session_state.set_tempo(
                            (audio_session_state.tempo() + 1.).min(999.),
                            link.clock_micros(),
                        );
                        link.commit_audio_session_state(&audio_session_state);
                    }
                    UpdateSessionState::TempoMinus => {
                        audio_session_state.set_tempo(
                            (audio_session_state.tempo() - 1.).max(20.),
                            link.clock_micros(),
                        );
                        link.commit_audio_session_state(&audio_session_state);
                    }
                    UpdateSessionState::TogglePlaying => {
                        if audio_session_state.is_playing() {
                            audio_session_state.set_is_playing(false, link.clock_micros() as u64);
                        } else {
                            audio_session_state.set_is_playing_and_request_beat_at_time(
                                true,
                                link.clock_micros() as u64,
                                0.,
                                last_known_quantum,
                            );
                        }
                        link.commit_audio_session_state(&audio_session_state);
                    }
                }
            }

            // ----  BUILD LATENCY COMPENSATED BUFFER & SYNTH SOUND ----

            let mut buffer: Vec<f32> = Vec::with_capacity(buffer_size);

            let begin_time = link.clock_micros() + output_latency as i64;

            for sample in 0..buffer_size {
                let mut y_amplitude: f32 = 0.;

                // Compute the host time for this sample and the last.
                let host_time = begin_time + (sample_time_micros * sample as f64).round() as i64;
                let last_sample_host_time = host_time - sample_time_micros.round() as i64;

                // Only make sound for positive beat magnitudes. Negative beat
                // magnitudes are count-in beats.
                if audio_session_state.beat_at_time(host_time, last_known_quantum) >= 0. {
                    // If the phase wraps around between the last sample and the
                    // current one with respect to a 1 beat quantum, then a click
                    // should occur.
                    if audio_session_state.phase_at_time(host_time, 1.0)
                        < audio_session_state.phase_at_time(last_sample_host_time, 1.0)
                    {
                        time_at_last_click = host_time;
                        synth_clock = 0; // reset synth clock
                    }

                    let micro_seconds_after_click = host_time - time_at_last_click;

                    // If we're within the click duration of the last beat, render
                    // the click tone into this sample
                    if micro_seconds_after_click < CLICK_DURATION {
                        // If the phase of the last beat with respect to the current
                        // quantum was zero, then it was at a quantum boundary and we
                        // want to use the high tone. For other beats within the
                        // quantum, use the low tone.
                        // dbg!(last_known_quantum);
                        let freq = match audio_session_state
                            .phase_at_time(host_time, last_known_quantum)
                            .floor() as usize
                        {
                            0 => HIGH_TONE,
                            _ => LOW_TONE,
                        };

                        let x_time = synth_clock as f32 / sample_rate as f32;

                        // Simple cosine synth:
                        y_amplitude =
                            (x_time * freq * TAU).cos() * (1. - (x_time * 2.5 * TAU).sin());

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

        Self { stream }
    }
}
