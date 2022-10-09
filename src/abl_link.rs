use crate::{rust_bindings::*, session_state::SessionState, split};

/// The representation of an abl_link instance.
pub struct AblLink {
    pub(crate) link: abl_link,
}

unsafe impl Send for AblLink {}
unsafe impl Sync for AblLink {}

impl Drop for AblLink {
    fn drop(&mut self) {
        // println!("Dropping AblLink");
        unsafe { abl_link_destroy(self.link) }
    }
}

impl AblLink {
    ///  Construct a new abl_link instance with an initial tempo.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    pub fn new(bpm: f64) -> AblLink {
        AblLink {
            link: unsafe { abl_link_create(bpm) },
        }
    }

    ///  Is Link currently enabled?
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: yes
    pub fn is_enabled(&self) -> bool {
        unsafe { abl_link_is_enabled(self.link) }
    }

    ///  Enable/disable Link.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    pub fn enable(&mut self, enable: bool) {
        unsafe { abl_link_enable(self.link, enable) }
    }

    ///  Is start/stop synchronization enabled?
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    pub fn is_start_stop_sync_enabled(&self) -> bool {
        unsafe { abl_link_is_start_stop_sync_enabled(self.link) }
    }

    ///  Enable start/stop synchronization.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    pub fn enable_start_stop_sync(&mut self, enable: bool) {
        unsafe { abl_link_enable_start_stop_sync(self.link, enable) }
    }

    ///  How many peers are currently connected in a Link session?
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: yes
    pub fn num_peers(&self) -> u64 {
        unsafe { abl_link_num_peers(self.link) }
    }

    /// Get the current link clock time in microseconds.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: yes
    pub fn clock_micros(&self) -> i64 {
        unsafe { abl_link_clock_micros(self.link) }
    }

    ///  Capture the current Link Session State from the audio thread.
    ///
    ///  Thread-safe: no
    ///
    ///  Realtime-safe: yes
    ///
    ///  This function should ONLY be called in the audio thread and must not be
    ///  accessed from any other threads. After capturing the session_state holds a snapshot
    ///  of the current Link Session State, so it should be used in a local scope. The
    ///  session_state should not be created on the audio thread.
    pub fn capture_audio_session_state(&self, session_state: &mut SessionState) {
        unsafe { abl_link_capture_audio_session_state(self.link, session_state.session_state) }
    }

    /// Capture the current Link Session State from an application thread.
    ///
    ///  Thread-safe: no
    ///
    ///  Realtime-safe: yes
    ///
    ///  Provides a mechanism for capturing the Link Session State from an
    ///  application thread (other than the audio thread). After capturing the session_state
    ///  contains a snapshot of the current Link state, so it should be used in a local
    ///  scope.
    pub fn capture_app_session_state(&self, session_state: &mut SessionState) {
        unsafe { abl_link_capture_app_session_state(self.link, session_state.session_state) };
    }

    ///  Commit the given Session State to the Link session from the audio thread.
    ///
    ///  Thread-safe: no
    ///
    ///  Realtime-safe: yes
    ///
    ///  This function should ONLY be called in the audio thread. The given
    ///  session_state will replace the current Link state. Modifications will be
    ///  communicated to other peers in the session.
    pub fn commit_audio_session_state(&mut self, session_state: &SessionState) {
        unsafe { abl_link_commit_audio_session_state(self.link, session_state.session_state) };
    }

    ///  Commit the given Session State to the Link session from an application thread.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    ///
    ///  The given session_state will replace the current Link Session State.
    ///  Modifications of the Session State will be communicated to other peers in the
    ///  session.
    pub fn commit_app_session_state(&mut self, session_state: &SessionState) {
        unsafe { abl_link_commit_app_session_state(self.link, session_state.session_state) };
    }

    ///  SAFETY: The callbacks/closures are handled by the underlying Link C++ library and may be run at any time.
    ///  Data races and hidden mutations can occur if a closure captures and uses local variables at the same
    ///  time as another thread.
    ///
    ///  Register a callback to be notified when the number of
    ///  peers in the Link session changes.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    ///
    ///  The callback is invoked on a Link-managed thread.
    pub fn set_num_peers_callback<C: FnMut(u64)>(&mut self, closure: &mut C) {
        unsafe {
            let (state, callback) = split::split_closure_trailing_data(closure);
            abl_link_set_num_peers_callback(self.link, Some(callback), state);
        }
    }

    ///  SAFETY: The callbacks/closures are handled by the underlying Link C++ library and may be run at any time.
    ///  Data races and hidden mutations can occur if a closure captures and uses local variables at the same
    ///  time as another thread.
    ///
    ///  Register a callback to be notified when the session tempo changes.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    ///
    ///  The callback is invoked on a Link-managed thread.
    pub fn set_tempo_callback<C: FnMut(f64)>(&mut self, closure: &mut C) {
        unsafe {
            let (state, callback) = split::split_closure_trailing_data(closure);
            abl_link_set_tempo_callback(self.link, Some(callback), state);
        }
    }

    ///  SAFETY: The callbacks/closures are handled by the underlying Link C++ library and may be run at any time.
    ///  Data races and hidden mutations can occur if a closure captures and uses local variables at the same
    ///  time as another thread.
    ///
    ///  Register a callback to be notified when the state of start/stop isPlaying changes.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    ///
    ///  The callback is invoked on a Link-managed thread.
    pub fn set_start_stop_callback<C: FnMut(bool)>(&mut self, closure: &mut C) {
        unsafe {
            let (state, callback) = split::split_closure_trailing_data(closure);
            abl_link_set_start_stop_callback(self.link, Some(callback), state);
        }
    }
}
