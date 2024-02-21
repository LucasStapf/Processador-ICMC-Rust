use async_channel::{Receiver, Sender};

use crate::ProcessorStatus;

pub struct ControlUnit {
    status_signal: (Sender<ProcessorStatus>, Receiver<ProcessorStatus>),
    // draw_signal: (Sender<()>, Receiver<()>),
}

impl Default for ControlUnit {
    fn default() -> Self {
        Self {
            status_signal: async_channel::bounded(1),
        }
    }
}
