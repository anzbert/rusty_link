// This example is a Rust port of 'link_hut' (written in C++ / with audio thread).
// Source: https://github.com/Ableton/link/tree/master/examples

use std::{
    io::{self, Write},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use audio_thread::AudioThread;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    queue,
    style::Print,
    terminal,
};
use rusty_link::{AblLink, SessionState};

mod audio_engine;
mod audio_thread;
mod constants;
mod synth;

fn main() {
    println!("\n\n < L I N K  H U T >\n");

    println!("usage:");
    println!("  enable / disable Link: a");
    println!("  start / stop: space");
    println!("  decrease / increase tempo: w / e");
    println!("  decrease / increase quantum: r / t");
    println!("  enable / disable start stop sync: s");
    println!("  quit: q");

    println!("\nenabled | num peers | quantum | start stop sync | tempo   | beats    | metro");

    let (input_tx, input_rx) = mpsc::channel::<InputCommand>();
    let input_thread = thread::spawn(move || {
        poll_input(input_tx);
    });

    let state = State::new(input_rx);

    let mut session_state = SessionState::new();

    '_main_loop: while state.running {
        let time = state.link.clock_micros();

        state.link.capture_app_session_state(&mut session_state);

        print_state(
            time,
            session_state,
            state.link.is_enabled(),
            state.link.num_peers(),
            state.audio_thread.engine.quantum(),
            state.audio_thread.engine.isStartStopSyncEnabled(),
        );

        std::thread::sleep(Duration::from_millis(10));
    }

    input_thread.join();
}

pub struct State {
    pub link: AblLink,
    pub running: bool,
    pub audio_thread: AudioThread,
}

impl State {
    pub fn new(input_rx: Receiver<InputCommand>) -> Self {
        let link = AblLink::new(120.);

        Self {
            link,
            running: true,
            audio_thread: AudioThread::new(&mut link, input_rx),
        }
    }
}

fn print_state(
    time: i64,
    state: SessionState,
    link_enabled: bool,
    num_peers: u64,
    quantum: f64,
    start_stop_sync_on: bool,
) {
    let enabled = match link_enabled {
        true => "yes",
        false => "no ",
    }
    .to_string();
    let start_stop = match start_stop_sync_on {
        true => "yes",
        false => "no ",
    };
    let playing = match state.is_playing() {
        true => "[playing]",
        false => "[stopped]",
    };
    let tempo = state.tempo();
    let beats = state.beat_at_time(time, quantum);
    let phase = state.phase_at_time(time, quantum);
    let mut metro = String::with_capacity(quantum as usize);
    for i in 0..quantum as usize {
        if i > phase as usize {
            metro.push('O');
        } else {
            metro.push('X');
        }
    }

    let mut stdout = io::stdout();
    queue!(
        stdout,
        cursor::SavePosition,
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print(format!("{:<7} | ", enabled)),
        Print(format!("{:<9} | ", num_peers)),
        Print(format!("{:<7} | ", quantum.trunc())),
        Print(format!("{:<3}   {:<9} | ", start_stop, playing)),
        Print(format!("{:<7.2} | ", tempo)),
        Print(format!("{:<8.2} | ", beats)),
        Print(format!("{}", metro)),
        cursor::RestorePosition,
    )
    .unwrap();
    stdout.flush().unwrap();
}

pub enum InputCommand {
    TempoPlus,
    TempoMinus,
    Quit,
    ToggleEnable,
    QuantumPlus,
    QuantumMinus,
    ToggleStartStopSync,
    TogglePlaying,
}

fn poll_input(tx: Sender<InputCommand>) {
    terminal::enable_raw_mode().unwrap();
    'input_loop: loop {
        if let Event::Key(event) = read().expect("Input read error") {
            match event.code {
                KeyCode::Char('a') => tx.send(InputCommand::ToggleEnable).unwrap(),
                KeyCode::Char('w') => tx.send(InputCommand::TempoMinus).unwrap(),
                KeyCode::Char('e') => tx.send(InputCommand::TempoPlus).unwrap(),
                KeyCode::Char('r') => tx.send(InputCommand::QuantumMinus).unwrap(),
                KeyCode::Char('t') => tx.send(InputCommand::QuantumPlus).unwrap(),
                KeyCode::Char('s') => tx.send(InputCommand::ToggleStartStopSync).unwrap(),
                KeyCode::Char(' ') => tx.send(InputCommand::TogglePlaying).unwrap(),
                KeyCode::Char('q') => {
                    tx.send(InputCommand::Quit).unwrap();
                    break 'input_loop;
                }
                _ => {}
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
}
