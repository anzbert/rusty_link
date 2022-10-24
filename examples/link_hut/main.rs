// This example is a Rust port of 'LinkHut' (original written in C++) with audio support.
// Source: https://github.com/Ableton/link/tree/master/examples
// See the cpal documentation (https://github.com/RustAudio/cpal) for ASIO and Jack support

use crate::{
    audio_engine::AudioEngine, audio_platform_cpal::AudioPlatformCpal,
    input_thread::UpdateSessionState,
};
use crossterm::{cursor, queue, style::Print, terminal};
use rusty_link::{AblLink, SessionState};
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::Duration,
};

mod audio_engine;
mod audio_platform_cpal;
mod input_thread;

fn main() {
    // Init Audio Device and print device info
    let audio_platform = AudioPlatformCpal::new();

    // Print Menu
    println!("\n < L I N K  H U T >\n");

    println!("usage:");
    println!("  enable / disable Link: a");
    println!("  start / stop: space");
    println!("  decrease / increase tempo: w / e");
    println!("  decrease / increase quantum: r / t");
    println!("  enable / disable start stop sync: s");
    println!("  quit: q");

    println!("\nenabled | num peers | quantum | start stop sync | tempo   | beats    | metro");

    // Init Multithread Variables
    let abl_link = Arc::new(AblLink::new(120.));
    let abl_link_clone_input_thread = Arc::clone(&abl_link);
    let abl_link_clone_audio_thread = Arc::clone(&abl_link);

    let running = Arc::new(AtomicBool::new(true));
    let running_clone_input_thread = Arc::clone(&running);

    let quantum = Arc::new(Mutex::new(4.));
    let quantum_clone_input_thread = Arc::clone(&quantum);
    let quantum_clone_audio_thread = Arc::clone(&quantum);

    // Init Terminal Input Thread
    let (input_tx, input_rx) = mpsc::channel::<UpdateSessionState>();
    let input_thread = thread::spawn(move || {
        input_thread::poll_input(
            input_tx,
            running_clone_input_thread,
            abl_link_clone_input_thread,
            quantum_clone_input_thread,
        );
    });

    // Init Audio Engine
    let mut audio_engine = AudioEngine::new(
        abl_link_clone_audio_thread,
        audio_platform,
        input_rx,
        quantum_clone_audio_thread,
    );

    // Crossterm UI Loop
    let mut app_session_state = SessionState::new();
    '_UI_loop: while running.load(Ordering::Acquire) {
        abl_link.capture_app_session_state(&mut app_session_state);
        print_state(
            abl_link.clock_micros(),
            &app_session_state,
            abl_link.is_enabled(),
            abl_link.num_peers(),
            *quantum.lock().unwrap(),
            abl_link.is_start_stop_sync_enabled(),
        );
        std::thread::sleep(Duration::from_millis(16)); // Frame Time 16ms = ~60fps
    }

    // Quit App
    abl_link.enable(false);
    audio_engine.stream = None;
    input_thread.join().unwrap();
}

/// Prints SessionState and AblLink Data to the terminal.
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
