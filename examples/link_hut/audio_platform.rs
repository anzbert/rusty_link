use std::sync::mpsc::Receiver;

use rusty_link::AblLink;

use crate::InputCommand;

pub struct AudioPlatform<'a> {
    input: Receiver<InputCommand>,
    link: &'a AblLink,
    engine_data: EngineData,
}

impl<'a> AudioPlatform<'a> {
    pub fn new(link: &AblLink, input: Receiver<InputCommand>) -> Self {
        Self {
            input,
            link,
            engine_data: EngineData {},
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
