use async_channel::{Receiver, Sender};
use log::error;
use once_cell::sync::Lazy;
use processor::Processor;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

pub enum RunMode {
    Run,
    Debug(bool),
}

pub struct ProcessadorICMC {
    tx: Sender<(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    )>,
    rx: Receiver<RunMode>,
    mode: RunMode,
    proc: Processor,
}

impl ProcessadorICMC {
    pub fn new(
        tx: Sender<(
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        )>,
        rx: Receiver<RunMode>,
    ) -> Self {
        Self {
            tx,
            rx,
            mode: RunMode::Debug(false),
            proc: Processor::new(),
        }
    }

    pub fn run(mut self) {
        RUNTIME.spawn(async move {
            loop {
                match self.rx.try_recv() {
                    Ok(m) => self.mode = m,
                    Err(e) => match e {
                        async_channel::TryRecvError::Empty => match self.mode {
                            RunMode::Run => match self.proc.next() {
                                Ok(_) => match self.tx.send(self.proc.state()).await {
                                    Ok(_) => (),
                                    Err(e) => {
                                        error!("{e}");
                                        break;
                                    }
                                },
                                Err(e) => {
                                    error!("{e}");
                                    break;
                                }
                            },
                            RunMode::Debug(b) => {
                                if b == true {
                                    match self.proc.next() {
                                        Ok(_) => match self.tx.send(self.proc.state()).await {
                                            Ok(_) => self.mode = RunMode::Debug(false),
                                            Err(e) => {
                                                error!("{e}");
                                                break;
                                            }
                                        },
                                        Err(e) => {
                                            error!("{e}");
                                            break;
                                        }
                                    }
                                }
                            }
                        },
                        async_channel::TryRecvError::Closed => {
                            error!("{e}");
                            break;
                        }
                    },
                }
            }
        });
    }
}
