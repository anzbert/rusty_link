use rusty_link::{AblLink, SessionState};
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    // Setup Ctrl-C Handler:
    let running = std::sync::Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Setup Link:
    let quantum = 4.0;
    let mut link = AblLink::new(120.0);
    let mut session_state = SessionState::new();
    link.enable(true);
    link.enable_start_stop_sync(true);

    // Callback Example:
    let mut closure = |value: bool| println!("is_playing: {} with quantum: {}", value, quantum);
    link.set_start_stop_callback(&mut closure);

    // Main Loop wrapped in Ctrl-C Handler:
    while running.load(Ordering::SeqCst) {
        link.capture_app_session_state(&mut session_state);

        let time = link.clock_micros();
        let tempo = session_state.tempo();
        let playing = session_state.is_playing();
        let beat = session_state.beat_at_time(time, quantum);
        println!(
            "playing={}, quantum={}, clock={}, tempo={}, beat={}",
            playing, quantum, time, tempo, beat
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Exit Routine:
    println!("Leaving Link session");
    link.enable(false);
}
