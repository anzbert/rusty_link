use crossterm::event::{poll, read, Event, KeyCode};
use rusty_link::{AblLink, SessionState};
use std::time::Duration;

pub struct State {
    pub link: AblLink,
    pub session_state: SessionState,
    pub running: bool,
    pub quantum: f64,
}

impl State {
    pub fn new() -> Self {
        Self {
            link: AblLink::new(100.),
            session_state: SessionState::new(),
            running: true,
            quantum: 4.,
        }
    }

    pub fn update_state(&mut self) {
        self.link.capture_app_session_state(&mut self.session_state);
    }
}

fn input(state: &mut State) -> crossterm::Result<()> {
    // `poll()` waits for an `Event` for a given time period
    if poll(Duration::from_millis(50))? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        if let Event::Key(event) = read()? {
            state.update_state();
            let tempo = state.session_state.tempo();
            let time_stamp = state.link.clock_micros();
            let enabled = state.link.is_enabled();

            match event.code {
                KeyCode::Char('q') => state.running = false,
                KeyCode::Char('a') => state.link.enable(!enabled),
                KeyCode::Char('w') => state.session_state.set_tempo(tempo - 1., time_stamp),
                KeyCode::Char('e') => state.session_state.set_tempo(tempo + 1., time_stamp),
                KeyCode::Char('r') => state.quantum -= 1.,
                KeyCode::Char('t') => state.quantum += 1.,
                KeyCode::Char('s') => state
                    .link
                    .enable_start_stop_sync(!state.link.is_start_stop_sync_enabled()),
                KeyCode::Char(' ') => {
                    if state.session_state.is_playing() {
                        state.session_state.set_is_playing(false, time_stamp as u64);
                    } else {
                        state.session_state.set_is_playing_and_request_beat_at_time(
                            true,
                            time_stamp as u64,
                            0.,
                            state.quantum,
                        )
                    }
                }
                _ => {}
            }
            state.link.commit_app_session_state(&state.session_state);
        }
    }

    Ok(())
}

fn main() {
    let mut state = State::new();

    // Callback Example:
    // state.link.set_start_stop_callback(|value: bool| {
    //     println!("is_playing: {} with quantum: {}", value, quantum)
    // });

    '_main_loop: while state.running {
        input(&mut state).expect("Input Err");
    }

    state.link.enable(false);
}
