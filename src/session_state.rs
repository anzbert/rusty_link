use crate::rust_bindings::*;

///  The representation of the current local state of a client in a Link Session.
///
///  A session state represents a timeline and the start/stop
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
pub struct SessionState {
    pub(crate) session_state: abl_link_session_state,
}

unsafe impl Send for SessionState {}
unsafe impl Sync for SessionState {}

impl Drop for SessionState {
    fn drop(&mut self) {
        // println!("Dropping SessionState");
        unsafe { abl_link_destroy_session_state(self.session_state) }
    }
}

impl SessionState {
    /// Create a new session_state instance.
    ///
    ///  Thread-safe: yes
    ///
    ///  Realtime-safe: no
    ///
    ///  The session_state is to be used with the abl_link_capture... and
    ///  abl_link_commit... functions to capture snapshots of the current link state and pass
    ///  changes to the link session.
    pub fn new() -> SessionState {
        unsafe {
            SessionState {
                session_state: abl_link_create_session_state(),
            }
        }
    }

    /// The tempo of the timeline, in Beats Per Minute.
    ///
    ///  This is a stable value that is appropriate for display to the user. Beat
    ///  time progress will not necessarily match this tempo exactly because of clock drift
    ///  compensation.
    pub fn tempo(&self) -> f64 {
        unsafe { abl_link_tempo(self.session_state) }
    }

    ///  Set the timeline tempo to the given bpm value, taking effect at the given time.
    pub fn set_tempo(&mut self, bpm: f64, at_time: i64) {
        unsafe { abl_link_set_tempo(self.session_state, bpm, at_time) }
    }

    ///  Get the beat value corresponding to the given time for the given quantum.
    ///
    ///  The magnitude of the resulting beat value is unique to this Link
    ///  client, but its phase with respect to the provided quantum is shared among all
    ///  session peers. For non-negative beat values, the following property holds:
    ///  ```fmod(beatAtTime(t, q), q) == phaseAtTime(t, q)```
    pub fn beat_at_time(&self, time: i64, quantum: f64) -> f64 {
        unsafe { abl_link_beat_at_time(self.session_state, time, quantum) }
    }

    /// Get the session phase at the given time for the given quantum.
    ///
    ///  The result is in the interval ```[0, quantum]```. The result is equivalent to
    ///  ```fmod(beatAtTime(t, q), q)``` for non-negative beat values. This function is convenient
    ///  if the client application is only interested in the phase and not the beat
    ///  magnitude. Also, unlike fmod, it handles negative beat values correctly.
    pub fn phase_at_time(&self, time: i64, quantum: f64) -> f64 {
        unsafe { abl_link_phase_at_time(self.session_state, time, quantum) }
    }

    ///  Get the time at which the given beat occurs for the given quantum.
    ///
    ///   The inverse of beatAtTime, assuming a constant tempo.
    ///  ```beatAtTime(timeAtBeat(b, q), q) === b```
    pub fn time_at_beat(&self, beat: f64, quantum: f64) -> i64 {
        unsafe { abl_link_time_at_beat(self.session_state, beat, quantum) }
    }

    /// Attempt to map the given beat to the given time in the context of the given quantum.
    ///
    /// This function behaves differently depending on the state of the
    ///  session. If no other peers are connected, then this abl_link instance is in a
    ///  session by itself and is free to re-map the beat/time relationship whenever it
    ///  pleases. In this case, ```beatAtTime(time, quantum) == beat``` after this funtion has been
    ///  called.
    ///
    ///  If there are other peers in the session, this abl_link instance should not abruptly
    ///  re-map the beat/time relationship in the session because that would lead to beat
    ///  discontinuities among the other peers. In this case, the given beat will be mapped
    ///  to the next time value greater than the given time with the same phase as the given
    ///  beat.
    ///
    ///  This function is specifically designed to enable the concept of "quantized launch"
    ///  in client applications. If there are no other peers in the session, then an event
    ///  (such as starting transport) happens immediately when it is requested. If there are
    ///  other peers, however, we wait until the next time at which the session phase matches
    ///  the phase of the event, thereby executing the event in-phase with the other peers in
    ///  the session. The client application only needs to invoke this function to achieve
    ///  this behavior and should not need to explicitly check the number of peers.
    pub fn request_beat_at_time(&mut self, beat: f64, time: i64, quantum: f64) {
        unsafe { abl_link_request_beat_at_time(self.session_state, beat, time, quantum) }
    }

    /// Rudely re-map the beat/time relationship for all peers in a session.
    ///
    ///  DANGER: This function should only be needed in certain special
    ///  circumstances. Most applications should not use it. It is very similar to
    ///  requestBeatAtTime except that it does not fall back to the quantizing behavior when
    ///  it is in a session with other peers. Calling this function will unconditionally map
    ///  the given beat to the given time and broadcast the result to the session. This is
    ///  very anti-social behavior and should be avoided.
    ///
    ///  One of the few legitimate uses of this function is to synchronize a Link session
    ///  with an external clock source. By periodically forcing the beat/time mapping
    ///  according to an external clock source, a peer can effectively bridge that clock into
    ///  a Link session. Much care must be taken at the application layer when implementing
    ///  such a feature so that users do not accidentally disrupt Link sessions that they may
    ///  join.
    pub fn force_beat_at_time(&mut self, beat: f64, time: u64, quantum: f64) {
        unsafe { abl_link_force_beat_at_time(self.session_state, beat, time, quantum) }
    }

    /// Set if transport should be playing or stopped, taking effect at the given time.
    pub fn set_is_playing(&mut self, is_playing: bool, time: u64) {
        unsafe { abl_link_set_is_playing(self.session_state, is_playing, time) }
    }

    /// Is transport playing?
    pub fn is_playing(&self) -> bool {
        unsafe { abl_link_is_playing(self.session_state) }
    }

    /// Get the time at which a transport start/stop occurs
    pub fn time_for_is_playing(&self) -> u64 {
        unsafe { abl_link_time_for_is_playing(self.session_state) }
    }

    /// Convenience function to attempt to map the given beat to the time
    /// when transport is starting to play in context of the given quantum.
    /// This function evaluates to a no-op if abl_link_is_playing equals false.
    pub fn request_beat_at_start_playing_time(&mut self, beat: f64, quantum: f64) {
        unsafe { abl_link_request_beat_at_start_playing_time(self.session_state, beat, quantum) }
    }

    /// Convenience function to start or stop transport at a given time and attempt
    /// to map the given beat to this time in context of the given quantum.
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
