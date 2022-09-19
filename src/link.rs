use crate::rust_bindings::*;
use crate::session_state::*;

///  ### The representation of an abl_link instance
///
///  Each abl_link instance has its own session state which
///  represents a beat timeline and a transport start/stop state. The
///  timeline starts running from beat 0 at the initial tempo when
///  constructed. The timeline always advances at a speed defined by
///  its current tempo, even if transport is stopped. Synchronizing to the
///  transport start/stop state of Link is optional for every peer.
///  The transport start/stop state is only shared with other peers when
///  start/stop synchronization is enabled.
///
///  An abl_link instance is initially disabled after construction, which
///  means that it will not communicate on the network. Once enabled,
///  an abl_link instance initiates network communication in an effort to
///  discover other peers. When peers are discovered, they immediately
///  become part of a shared Link session.
///
///  Each function documents its thread-safety and
///  realtime-safety properties. When a function is marked thread-safe,
///  it means it is safe to call from multiple threads
///  concurrently. When a function is marked realtime-safe, it means that
///  it does not block and is appropriate for use in the thread that
///  performs audio IO.
///
///  One session state capture/commit function pair for use
///  in the audio thread and one for all other application contexts is provided.
///  In general, modifying the session state should be done in the audio
///  thread for the most accurate timing results. The ability to modify
///  the session state from application threads should only be used in
///  cases where an application's audio thread is not actively running
///  or if it doesn't generate audio at all. Modifying the Link session
///  state from both the audio thread and an application thread
///  concurrently is not advised and will potentially lead to unexpected
///  behavior.
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

    ///  Commit the given Session State to the Link session from the audio thread.
    ///
    ///  Thread-safe: no
    ///
    ///  Realtime-safe: yes
    ///
    ///  This function should ONLY be called in the audio thread. The given
    ///  session_state will replace the current Link state. Modifications will be
    ///  communicated to other peers in the session.
    pub fn commit_audio_session_state(&mut self, ss: &SessionState) {
        unsafe { abl_link_commit_audio_session_state(self.link, ss.session_state) }
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
    pub fn commit_app_session_state(&mut self, ss: &SessionState) {
        unsafe { abl_link_commit_app_session_state(self.link, ss.session_state) }
    }

    // CALLBACKS:

    // pub fn set_num_peers_callback(&mut self, callback: abl_link_num_peers_callback) {
    //     unsafe {
    //         abl_link_set_num_peers_callback(self.link, callback, context);
    //         todo!();
    //         // let cb = callback as unsafe extern "C" fn(size_t);
    //         // Link_setNumPeersCallback(self.link, Some(cb));
    //     }
    // }

    // pub fn set_tempo_callback(&mut self, callback: extern "C" fn(f64)) {
    //     unsafe {
    //         let cb = callback as unsafe extern "C" fn(f64);
    //         Link_setTempoCallback(self.link, Some(cb));
    //     }
    // }

    // pub fn set_start_stop_callback(&mut self, callback: unsafe extern "C" fn(bool)) {
    //     extern "C" fn cb(is_playing: bool, null: *mut std::os::raw::c_void) {
    //         callback(is_playing);
    //     }

    //     // let test = fn(is_playing: bool, context: *mut ::std::os::raw::c_void);
    //     unsafe {
    //         // let cb = callback as unsafe extern "C" fn(bool);
    //         // let cb1 = callback as unsafe extern "C" fn(bool, *mut std::os::raw::c_void);
    //         // abl_link_start_stop_callback;
    //         abl_link_set_start_stop_callback(self.link, Some(cb), 0 as *mut std::os::raw::c_void);
    //     }
    // }
}
