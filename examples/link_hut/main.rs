// This example is a Rust port of 'link_hut' (written in C++ / with audio thread).
// Source: https://github.com/Ableton/link/tree/master/examples

use crate::{audio_engine::AudioEngine, audio_platform_cpal::AudioPlatformCpal};
use cpal::traits::StreamTrait;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    queue,
    style::Print,
    terminal,
};
use rusty_link::{AblLink, SessionState};
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

mod audio_engine;
mod audio_platform_cpal;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref ABL_LINK: AblLink = AblLink::new(120.);
}

fn main() {
    let audio_platform = AudioPlatformCpal::new();

    println!("\n < L I N K  H U T >\n");

    println!("usage:");
    println!("  enable / disable Link: a");
    println!("  start / stop: space");
    println!("  decrease / increase tempo: w / e");
    println!("  decrease / increase quantum: r / t");
    println!("  enable / disable start stop sync: s");
    println!("  quit: q");

    println!("\nenabled | num peers | quantum | start stop sync | tempo   | beats    | metro");
    // App running:
    let running = Arc::new(AtomicBool::new(true));
    let running_clone1 = Arc::clone(&running);

    let quantum = Arc::new(Mutex::new(4.));
    let quantum_clone1 = Arc::clone(&quantum);
    let quantum_clone2 = Arc::new(Mutex::new(4.));

    // Init Input Thread:
    let (input_tx, input_rx) = mpsc::channel::<UpdateSessionState>();
    let input_thread = thread::spawn(move || {
        poll_input(input_tx, running_clone1, &ABL_LINK, quantum_clone1);
    });

    // Init Main State
    // let link: AblLink = AblLink::new(120.);
    let audio_engine = AudioEngine::new(&ABL_LINK, audio_platform, input_rx, quantum_clone2);

    // UI
    let mut app_session_state = SessionState::new();

    '_UI_loop: while running.load(Ordering::Acquire) {
        ABL_LINK.capture_app_session_state(&mut app_session_state);
        print_state(
            ABL_LINK.clock_micros(),
            &app_session_state,
            ABL_LINK.is_enabled(),
            ABL_LINK.num_peers(),
            *quantum.lock().unwrap(),
            ABL_LINK.is_start_stop_sync_enabled(),
        );
        std::thread::sleep(Duration::from_millis(10));
    }

    // Exit App
    audio_engine.stream.pause().unwrap();
    input_thread.join().unwrap();
}

pub enum UpdateSessionState {
    TempoPlus,
    TempoMinus,
    TogglePlaying,
}

fn poll_input(
    tx: Sender<UpdateSessionState>,
    running: Arc<AtomicBool>,
    link: &AblLink,
    quantum: Arc<Mutex<f64>>,
) {
    terminal::enable_raw_mode().unwrap();
    'input_loop: loop {
        if let Event::Key(event) = read().expect("Input read error") {
            match event.code {
                KeyCode::Char('w') => tx.send(UpdateSessionState::TempoMinus).unwrap(),
                KeyCode::Char('e') => tx.send(UpdateSessionState::TempoPlus).unwrap(),
                KeyCode::Char(' ') => tx.send(UpdateSessionState::TogglePlaying).unwrap(),
                KeyCode::Char('a') => {
                    link.enable(!link.is_enabled());
                }
                KeyCode::Char('r') => {
                    let mut q = quantum.lock().unwrap();
                    *q = (*q - 1.).max(1.);
                }
                KeyCode::Char('t') => {
                    let mut q = quantum.lock().unwrap();
                    *q = (*q + 1.).min(16.);
                }
                KeyCode::Char('s') => {
                    link.enable_start_stop_sync(!link.is_start_stop_sync_enabled());
                }
                KeyCode::Char('q') => {
                    running.store(false, Ordering::Release);
                    break 'input_loop;
                }
                _ => {}
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
    println!("\n");
}

fn print_state(
    time: i64,
    state: &SessionState,
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
