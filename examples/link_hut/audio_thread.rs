use crate::InputCommand;
use rusty_link::AblLink;
use std::sync::mpsc::Receiver;

pub struct AudioThread {
    pub quantum: f64,
}

impl AudioThread {
    pub fn new(link: &mut AblLink, input_rx: Receiver<InputCommand>) -> Self {
        Self { quantum: 4. }
    }
}
