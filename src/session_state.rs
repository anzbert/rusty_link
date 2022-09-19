use crate::link::*;
use crate::rust_bindings::*;

// Potentially separate functionality of Audio and App States:
// struct Audio;
// struct App;

// trait StateType {}
// impl StateType for Audio {}
// impl StateType for App {}

// struct SessState<T: StateType> {
//     session_state: abl_link_session_state,
//     state_type: T,
// }

// impl<T: StateType> SessState<T> {
//     // general
// }
// impl SessState<Audio> {}
// impl SessState<App> {}

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub enum State {
//     Audio,
//     App,
//     New,
// }

/// @brief The representation of the current local state of a client in a Link Session
///
///  @discussion A session state represents a timeline and the start/stop
///  state. The timeline is a representation of a mapping between time and
///  beats for varying quanta. The start/stop state represents the user
///  intention to start or stop transport at a specific time. Start stop
///  synchronization is an optional feature that allows to share the user
///  request to start or stop transport between a subgroup of peers in a
///  Link session. When observing a change of start/stop state, audio
///  playback of a peer should be started or stopped the same way it would
///  have happened if the user had requested that change at the according
///  time locally. The start/stop state can only be changed by the user.
///  This means that the current local start/stop state persists when
///  joining or leaving a Link session. After joining a Link session
///  start/stop change requests will be communicated to all connected peers.
///
pub struct SessionState {
    pub(crate) session_state: abl_link_session_state,
    // state_type: State,
}

impl Drop for SessionState {
    fn drop(&mut self) {
        // println!("Dropping SessionState");
        unsafe { abl_link_destroy_session_state(self.session_state) }
    }
}

impl SessionState {
    pub fn new() -> SessionState {
        unsafe {
            SessionState {
                session_state: abl_link_create_session_state(),
                // state_type: State::New,
            }
        }
    }

    // pub fn state_type(&self) -> State {
    //     self.state_type
    // }

    pub fn capture_app_session_state(&mut self, link: &AblLink) {
        // self.state_type = State::App;
        unsafe { abl_link_capture_app_session_state(link.link, self.session_state) }
    }

    pub fn capture_audio_session_state(&mut self, link: &AblLink) {
        // self.state_type = State::Audio;
        unsafe { abl_link_capture_audio_session_state(link.link, self.session_state) }
    }

    pub fn tempo(&self) -> f64 {
        unsafe { abl_link_tempo(self.session_state) }
    }

    pub fn set_tempo(&mut self, bpm: f64, at_time: i64) {
        unsafe { abl_link_set_tempo(self.session_state, bpm, at_time) }
    }

    pub fn beat_at_time(&self, time: i64, quantum: f64) -> f64 {
        unsafe { abl_link_beat_at_time(self.session_state, time, quantum) }
    }

    pub fn phase_at_time(&self, time: i64, quantum: f64) -> f64 {
        unsafe { abl_link_phase_at_time(self.session_state, time, quantum) }
    }

    pub fn time_at_beat(&self, beat: f64, quantum: f64) -> i64 {
        unsafe { abl_link_time_at_beat(self.session_state, beat, quantum) }
    }

    pub fn request_beat_at_time(&mut self, beat: f64, time: i64, quantum: f64) {
        unsafe { abl_link_request_beat_at_time(self.session_state, beat, time, quantum) }
    }

    pub fn force_beat_at_time(&mut self, beat: f64, time: u64, quantum: f64) {
        unsafe { abl_link_force_beat_at_time(self.session_state, beat, time, quantum) }
    }

    pub fn set_is_playing(&mut self, is_playing: bool, time: u64) {
        unsafe { abl_link_set_is_playing(self.session_state, is_playing, time) }
    }

    pub fn is_playing(&self) -> bool {
        unsafe { abl_link_is_playing(self.session_state) }
    }

    pub fn time_for_is_playing(&self) -> u64 {
        unsafe { abl_link_time_for_is_playing(self.session_state) }
    }

    pub fn request_beat_at_start_playing_time(&mut self, beat: f64, quantum: f64) {
        unsafe { abl_link_request_beat_at_start_playing_time(self.session_state, beat, quantum) }
    }

    pub fn set_is_playing_and_request_beat_at_time(
        &mut self,
        is_playing: bool,
        time: u64,
        beat: f64,
        quantum: f64,
    ) {
        unsafe {
            abl_link_set_is_playing_and_request_beat_at_time(
                self.session_state,
                is_playing,
                time,
                beat,
                quantum,
            )
        }
    }
}
