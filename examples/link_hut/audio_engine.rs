use crate::{audio_platform_cpal::AudioPlatformCpal, mono_sine::MonoSine, InputCommand};
use cpal::Stream;
use rusty_link::{AblLink, SessionState};
use std::sync::mpsc::Receiver;

pub const LOW_TONE: f32 = 261.63; // C
pub const HIGH_TONE: f32 = 392.00; // G
pub const CLICK_DURATION: u64 = 100_000; // 100 ms click duration in micros

pub struct AudioEngine {
    input: Receiver<InputCommand>,
    pub link: &'static AblLink,
    engine_data: EngineData,
    pub quantum: f64,
    audio_platorm: AudioPlatformCpal,
    stream: Stream,
}

impl AudioEngine {
    pub fn new(link: &'static AblLink, input: Receiver<InputCommand>) -> Self {
        let audio_cpal = AudioPlatformCpal::new();

        let mut audio_session_state = SessionState::new();

        let mut low_sine = MonoSine::new(audio_cpal.config.sample_rate.0, LOW_TONE);
        let mut high_sine = MonoSine::new(audio_cpal.config.sample_rate.0, HIGH_TONE);

        let mut engine_callback = move |buffer_size: u64, latency_in_samples: u64| {
            // handle audio session state:
            // let engineData = pullEngineData();
            // let audio_session_state = link.capture_audio_session_state(&mut audio_session_state);

            // build a latency compensated buffer:

            let mut buffer: Vec<f32> = Vec::new();

            buffer
        };

        let stream_callback =
            AudioPlatformCpal::build_callback::<f32>(audio_cpal.config.clone(), engine_callback);

        let stream = audio_cpal.build_stream(stream_callback);

        Self {
            link,
            input,
            engine_data: EngineData {},
            quantum: 4.,
            audio_platorm: audio_cpal,
            stream,
        }
    }

    fn now(&self) -> i64 {
        self.link.clock_micros()
    }

    pub fn start_playing(&mut self) {
        let mut session_state = SessionState::new();
        self.link.capture_app_session_state(&mut session_state);
        session_state.set_is_playing_and_request_beat_at_time(
            true,
            self.now() as u64,
            0.,
            self.quantum,
        );
        self.link.commit_app_session_state(&session_state);
    }

    pub fn stop_playing(&mut self) {
        let mut session_state = SessionState::new();
        self.link.capture_app_session_state(&mut session_state);
        session_state.set_is_playing(true, self.now() as u64);
        self.link.commit_app_session_state(&session_state);
    }

    pub fn is_playing(&self) -> bool {
        let mut session_state = SessionState::new();
        self.link.capture_app_session_state(&mut session_state);
        session_state.is_playing()
    }

    pub fn beat_time(&self) -> f64 {
        let mut session_state = SessionState::new();
        self.link.capture_app_session_state(&mut session_state);
        session_state.beat_at_time(self.now(), self.quantum)
    }

    pub fn set_tempo(&mut self, tempo: f64) {
        let mut session_state = SessionState::new();
        self.link.capture_app_session_state(&mut session_state);
        session_state.set_tempo(tempo, self.now());
        self.link.commit_app_session_state(&session_state);
    }

    pub fn quantum(&self) -> f64 {
        self.quantum
    }

    pub fn set_quantum(&mut self, quantum: f64) {
        self.quantum = quantum;
    }

    pub fn is_start_stop_sync_enabled(&self) -> bool {
        self.link.is_start_stop_sync_enabled()
    }

    pub fn set_start_stop_sync_enabled(&mut self, enabled: bool) {
        self.link.enable_start_stop_sync(enabled);
    }
}

struct EngineData {}
