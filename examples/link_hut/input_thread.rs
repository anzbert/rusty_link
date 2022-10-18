use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use rusty_link::AblLink;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
};

pub enum UpdateSessionState {
    TempoPlus,
    TempoMinus,
    TogglePlaying,
}

pub fn poll_input(
    tx: mpsc::Sender<UpdateSessionState>,
    running: Arc<AtomicBool>,
    link: &AblLink,
    quantum: Arc<Mutex<f64>>,
) {
    terminal::enable_raw_mode().unwrap();
    'input_loop: loop {
        if let Event::Key(event) = event::read().expect("Input read error") {
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
