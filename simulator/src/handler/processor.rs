use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

use processor::errors::ProcessorError;

pub enum State {
    Stopped,
    Debug,
    Running,
}

pub struct ProcessorHandler {
    processor: Arc<Mutex<processor::Processor>>,
    state_channel: (async_channel::Sender<State>, async_channel::Receiver<State>),
    error_channel: (
        async_channel::Sender<ProcessorError>,
        async_channel::Receiver<ProcessorError>,
    ),
}

impl Default for ProcessorHandler {
    fn default() -> Self {
        Self {
            processor: Default::default(),
            state_channel: async_channel::bounded(1),
            error_channel: async_channel::bounded(1),
        }
    }
}

impl ProcessorHandler {
    pub fn start(&self) {
        let recv = self.state_channel.1.clone();
        let send = self.error_channel.0.clone();
        let p = self.processor.clone();

        thread::spawn(move || {
            let mut state = State::Stopped;
            loop {
                match state {
                    State::Stopped => match recv.recv_blocking() {
                        Ok(s) => state = s,
                        Err(e) => {
                            log::error!("{}", e);
                            break;
                        }
                    },
                    State::Debug => state = State::Stopped,
                    State::Running => (),
                }

                match p.lock() {
                    Ok(mut p) => match p.instruction_cicle() {
                        Ok(_) => (),
                        Err(e) => {
                            send.send_blocking(e);
                            break;
                        }
                    },
                    Err(e) => {
                        log::error!("{}", e);
                        break;
                    }
                }
            }
        });
    }
}
