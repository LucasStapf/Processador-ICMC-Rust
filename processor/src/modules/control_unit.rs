use std::sync::{Arc, Condvar, Mutex};

use async_channel::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct ControlUnit {
    run_signal_send: Sender<bool>,
    run_signal_recv: Receiver<bool>,
}

impl Default for ControlUnit {
    fn default() -> Self {
        let (run_signal_send, run_signal_recv) = async_channel::bounded(1);
        Self {
            run_signal_send,
            run_signal_recv,
        }
    }
}

impl ControlUnit {
    pub fn run_signal_send(&self) -> Sender<bool> {
        self.run_signal_send.clone()
    }

    pub fn run_signal_recv(&self) -> Receiver<bool> {
        self.run_signal_recv.clone()
    }
}
