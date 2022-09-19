use rusty_link::{AblLink, SessionState, TestStruct};
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

extern "C" fn test(is_playing: bool, link_ptr: *mut c_void) {
    let back_to_rust_ref = unsafe { &mut *(link_ptr as *mut AblLink) };
    // let ptr = link_ptr as *mut Option<AblLink>;
    println!("XXXXXXXX link: {:?}", back_to_rust_ref.clock_micros());

    println!("YOYOYOYO play state: {}", is_playing);
}

extern "C" fn test2(is_playing: bool, link_ptr: *mut c_void) {
    let back_to_rust_ref = unsafe { &mut *(link_ptr as *mut TestStruct) };
    // let ptr = link_ptr as *mut Option<AblLink>;
    println!("XXXXXXXX link: {:?}", back_to_rust_ref.number);

    println!("YOYOYOYO play state: {}", is_playing);
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let quantum = 4.0;
    let mut link = AblLink::new(120.0);
    let mut session_state = SessionState::new();

    link.enable(true);
    link.enable_start_stop_sync(true);

    link.set_start_stop_callback(test);

    // let mut test_struct = TestStruct { number: 99 };
    // link.set_test_callback(test2, &mut test_struct);

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
