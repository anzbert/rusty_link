use rusty_link::{Link, SessionState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let quantum = 4.0;
    let mut link = Link::new(120.0);
    let mut session_state = SessionState::new();

    link.enable(true);
    link.enable_start_stop_sync(true);

    while running.load(Ordering::SeqCst) {
        session_state.capture_app_session_state(&link);

        let time = link.clock_micros();
        let tempo = session_state.tempo();
        let playing = session_state.is_playing();
        let beat = session_state.beat_at_time(time, quantum);
        println!(
            "playing={}, quantum={}, clock={}, tempo={}, beat={}",
            playing, quantum, time, tempo, beat
        );
        thread::sleep(Duration::from_millis(100));
    }

    println!("Leaving Link session");
    link.enable(false);
}
