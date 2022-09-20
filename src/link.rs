//! # Rusty Link
//!
//! Rusty Link is a Rust wrapper of [abl_link](https://github.com/Ableton/link/tree/master/extensions/abl_link), which
//! is a C 11 wrapper made by Ableton for their C++ codebase.
//! This library attempts to be unopinionated and plain in
//! copying the functionality of abl_link, while providing Rust's safety guarantees.
//!
//! [Ableton Link](http://ableton.github.io/link) is a technology that synchronizes musical beat, tempo,
//! phase, and start/stop commands across multiple applications running
//! on one or more devices. Applications on devices connected to a local
//! network discover each other automatically and form a musical session
//! in which each participant can perform independently: anyone can start
//! or stop while still staying in time. Anyone can change the tempo, the
//! others will follow. Anyone can join or leave without disrupting the session.
//!
//! ## Implementation
//!
//! - Rusty Link currently wraps around all functions available in ['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) and makes them publicly available, except for the destructors, which are implemented on the Drop trait.
//! - The 'create' functions for abl_link and session_state have been renamed to 'new' to make the API more Rust-intuitive.
//! - Functions have been implemented as methods on either the AblLink or the SessionState struct depending on which of the two the original C function mutates or uses as a parameter.
//! - At this point, handling thread and realtime safety with Audio and App Session States is left up to the user, just like in the original library.
//! - Ableton's documentation should mostly still apply to this library, since implementations have been copied as they were.
//! - The function documentations have been copied from 'abl_link.h', except for the addition of the following safety warning for callbacks.
//!
//! ## Safety
//!
//! The callbacks/closures are handled by the underlying Link C++ library and may be run at any time.
//! Data races and hidden mutations can occur if a closure captures and uses local variables at the same
//! time as another thread.
//!
//! ## Credits
//!
//! Thanks to Magnus Herold for [his implementation](https://github.com/magdaddy/ableton-link-rs).
//! This library started as a fork of his, but is now purely built on Ableton's basic C Wrapper.

use crate::rust_bindings::*;
use crate::session_state::*;

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
            let (state, callback) = ffi_helpers::split_closure(closure);
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
            let (state, callback) = ffi_helpers::split_closure(closure);
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
            let (state, callback) = ffi_helpers::split_closure(closure);
            abl_link_set_start_stop_callback(self.link, Some(callback), state);
        }
    }
}
