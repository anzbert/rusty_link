use crate::{audio_platform_cpal::AudioPlatformCpal, input_thread::UpdateSessionState};
use cpal::Stream;
use rusty_link::{AblLink, HostTimeFilter, SessionState};
use std::{
    cmp::Ordering,
    f32::consts::TAU,
    sync::{mpsc::Receiver, Arc, Mutex},
    time::Duration,
};

const HIGH_TONE: f32 = 1567.98; // G
const LOW_TONE: f32 = 1108.73; // C#
const CLICK_DURATION: u64 = 100_000; // in microseconds

pub struct AudioEngine {
    pub stream: Option<Stream>,
}

impl AudioEngine {
    pub fn new(
        link: &'static AblLink,
        audio_cpal: AudioPlatformCpal,
        input: Receiver<UpdateSessionState>,
        quantum: Arc<Mutex<f64>>,
    ) -> Self {
        let mut host_time_filter = HostTimeFilter::new();
        let mut audio_session_state = SessionState::new();

        let mut sample_time: u64 = 0;
        let mut synth_clock: u64 = 0;
        let mut time_at_last_click = Duration::from_secs(0);

        let mut last_known_quantum = *quantum.lock().unwrap();

        let engine_callback = move |buffer_size: usize,
                                    sample_rate: u64,
                                    output_latency: Duration,
                                    sample_time_micros: Duration| {
            let invoke_time =
                host_time_filter.sample_time_to_host_time(link.clock_micros(), sample_time);

            let invoke_time_as_duration = Duration::from_micros(invoke_time as u64);

            // ---- HANDLE AUDIO SESSION STATE ----
            link.capture_audio_session_state(&mut audio_session_state);

            if let Ok(q) = quantum.try_lock() {
                last_known_quantum = *q;
            };

            if let Ok(command) = input.try_recv() {
                match command {
                    UpdateSessionState::TempoPlus => {
                        audio_session_state
                            .set_tempo((audio_session_state.tempo() + 1.).min(999.), invoke_time);
                        link.commit_audio_session_state(&audio_session_state);
                    }
                    UpdateSessionState::TempoMinus => {
                        audio_session_state
                            .set_tempo((audio_session_state.tempo() - 1.).max(20.), invoke_time);
                        link.commit_audio_session_state(&audio_session_state);
                    }
                    UpdateSessionState::TogglePlaying => {
                        if audio_session_state.is_playing() {
                            audio_session_state.set_is_playing(false, invoke_time as u64);
                        } else {
                            audio_session_state.set_is_playing_and_request_beat_at_time(
                                true,
                                invoke_time as u64,
                                0.,
                                last_known_quantum,
                            );
                        }
                        link.commit_audio_session_state(&audio_session_state);
                    }
                }
            }

            // ----  BUILD LATENCY COMPENSATED BUFFER WITH SYNTH SOUND ----

            let mut buffer: Vec<f32> = Vec::with_capacity(buffer_size);

            let begin_time = invoke_time_as_duration + output_latency;

            for sample in 0..buffer_size {
                if !audio_session_state.is_playing() {
                    buffer.push(0.);
                    continue;
                }

                let mut y_amplitude: f32 = 0.; // Default is silent

                // Compute the host time for this sample and the last.
                let host_time = begin_time + (sample_time_micros * sample as u32);
                let last_sample_host_time = host_time - sample_time_micros;

                // Only make sound for positive beat magnitudes. Negative beat
                // magnitudes are count-in beats.
                if audio_session_state
                    .beat_at_time(host_time.as_micros() as i64, last_known_quantum)
                    >= 0.
                {
                    // If the phase wraps around between the last sample and the
                    // current one with respect to a 1 beat quantum, then a click
                    // should occur.
                    if audio_session_state.phase_at_time(host_time.as_micros() as i64, 1.0)
                        < audio_session_state
                            .phase_at_time(last_sample_host_time.as_micros() as i64, 1.0)
                    {
                        time_at_last_click = host_time; // reset last click time
                        synth_clock = 0; // reset synth clock
                    }

                    let micro_seconds_after_click = host_time - time_at_last_click;

                    // If we're within the click duration of the last beat, render
                    // the click tone into this sample
                    if let Ordering::Less =
                        micro_seconds_after_click.cmp(&Duration::from_millis(CLICK_DURATION))
                    {
                        // If the phase of the last beat with respect to the current
                        // quantum was zero, then it was at a quantum boundary and we
                        // want to use the high tone. For other beats within the
                        // quantum, use the low tone.
                        let freq = match audio_session_state
                            .phase_at_time(host_time.as_micros() as i64, last_known_quantum)
                            .floor() as usize
                        {
                            0 => HIGH_TONE,
                            _ => LOW_TONE,
                        };

                        let x_time = synth_clock as f32 / sample_rate as f32;

                        y_amplitude =
                            (x_time * freq * TAU).cos() * (1. - (x_time * 2.5 * TAU).sin()); // Simple cosine synth

                        // y_amplitude = (x_time * freq * TAU).sin(); // Alternatively, a simple sine synth

                        synth_clock = (synth_clock + 1) % sample_rate;
                    }
                }
                buffer.push(y_amplitude);
            }

            sample_time += buffer_size as u64;
            buffer
        };

        // BUILD AUDIO STREAM AND START
        let stream = audio_cpal.build_stream::<f32>(engine_callback);

        Self {
            stream: Some(stream),
        }
    }
}
