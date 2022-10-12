use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    queue,
    style::Print,
    terminal,
};
use rusty_link::{AblLink, SessionState};
use std::{
    io::{stdout, Write},
    time::Duration,
};

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

fn print_state(state: &mut State) {
    state.update_state();

    let time = state.link.clock_micros();
    let enabled = match state.link.is_enabled() {
        true => "yes",
        false => "no",
    }
    .to_string();
    let num_peers = state.link.num_peers();
    let start_stop = match state.link.is_start_stop_sync_enabled() {
        true => "yes",
        false => "no",
    };
    let playing = match state.session_state.is_playing() {
        true => "yes",
        false => "no",
    };
    let tempo = state.session_state.tempo();
    let beats = state.session_state.beat_at_time(time, state.quantum);
    let phase = state.session_state.phase_at_time(time, state.quantum);
    let mut metro = String::with_capacity(state.quantum as usize);
    for i in 0..state.quantum as usize {
        if i < phase as usize {
            metro.push('X');
        } else {
            metro.push('O');
        }
    }

    let mut stdout = stdout();
    queue!(
        stdout,
        Print(format!("{} | ", enabled)),
        Print(format!("{} | ", num_peers)),
        Print(format!("{} | ", state.quantum)),
        Print(format!("{} {} | ", start_stop, playing)),
        Print(format!("{} | ", tempo)),
        Print(format!("{} | ", beats)),
        Print(format!("{}", metro)),
        cursor::MoveUp(1),
    )
    .unwrap();
    stdout.flush().unwrap();
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
                KeyCode::Char('w') => state
                    .session_state
                    .set_tempo((tempo - 1.).clamp(20., 200.), time_stamp),
                KeyCode::Char('e') => state
                    .session_state
                    .set_tempo((tempo + 1.).clamp(20., 200.), time_stamp),
                KeyCode::Char('r') => state.quantum = (state.quantum - 1.).clamp(0., 8.),
                KeyCode::Char('t') => state.quantum = (state.quantum + 1.).clamp(0., 8.),
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
    terminal::enable_raw_mode().unwrap();

    // Callback Example:
    // state.link.set_start_stop_callback(|value: bool| {
    //     println!("is_playing: {} with quantum: {}", value, quantum)
    // });

    '_main_loop: while state.running {
        input(&mut state).expect("Input Err");
        print_state(&mut state);
    }

    state.link.enable(false);
    terminal::disable_raw_mode().unwrap();
}
