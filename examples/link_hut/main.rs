// This example is a Rust port of 'link_hut' (written in C++ / with audio thread).
// Source: https://github.com/Ableton/link/tree/master/examples

mod audio;

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

    // terminal::enable_raw_mode().unwrap();

    // '_main_loop: while state.running {
    // poll_input(&mut state).expect("Input Fn Error");
    // print_state(&mut state);
    // }

    // terminal::disable_raw_mode().unwrap();
}
