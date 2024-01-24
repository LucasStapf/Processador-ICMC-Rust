mod instructions;

use async_channel::{Receiver, Sender};
use log::error;
use once_cell::sync::Lazy;
use processor::Processor;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

#[derive(Clone)]
pub enum RunMode {
    Run,
    Debug(bool),
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Debug(false)
    }
}

#[derive(Clone)]
pub struct ProcessorManager {
    pub mode: Arc<Mutex<RunMode>>,
    pub processor: Arc<Mutex<Processor>>,
    pub tx: Option<Sender<()>>,
    pub rx: Option<Receiver<bool>>,
}

impl Default for ProcessorManager {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            processor: Default::default(),
            tx: None,
            rx: None,
        }
    }
}

impl ProcessorManager {
    pub fn new(tx: Option<Sender<()>>, rx: Option<Receiver<bool>>) -> Self {
        let mut pm = ProcessorManager::default();
        pm.tx = tx;
        pm.rx = rx;
        pm
    }

    // pub fn set_mode(&mut self, mode: RunMode) -> Result<(), ProcError> {
    //     match self.mode.lock() {
    //         Ok(mut m) => {
    //             *m = mode;
    //             Ok(())
    //         }
    //         Err(e) => {
    //             warn!("{e}");
    //             Err(ProcError::BlockedMemory)
    //         }
    //     }
    // }

    // pub fn mem(&self) -> Result<Arc<Mutex<Vec<MemoryCell>>>, ProcError> {
    //     match self.processor.try_lock() {
    //         Ok(p) => Ok(p.arc_mem()),
    //         Err(e) => {
    //             warn!("{e}");
    //             Err(ProcError::BlockedMemory)
    //         }
    //     }
    // }

    pub fn run(&self) {
        if let Some(tx) = self.tx.clone() {
            let mode = self.mode.clone();
            let processor = self.processor.clone();
            RUNTIME.spawn(async move {
                loop {
                    let mut bool_mode = false;
                    let mut bool_error = false;
                    match mode.lock() {
                        Ok(mut m) => match *m {
                            RunMode::Run => (),
                            RunMode::Debug(b) => match b {
                                true => {
                                    *m = RunMode::Debug(false);
                                    bool_mode = true;
                                }
                                false => continue,
                            },
                        },
                        Err(e) => {
                            error!("{e}");
                            bool_error = true;
                        }
                    }

                    if bool_error {
                        match tx.send(()).await {
                            Ok(_) => break,
                            Err(e) => {
                                error!("{e}");
                                break;
                            }
                        }
                    }

                    match processor.lock() {
                        Ok(mut p) => match p.next() {
                            Ok(_) => (),
                            Err(e) => {
                                error!("{e}");
                                bool_error = true;
                            }
                        },
                        Err(e) => {
                            error!("{e}");
                            bool_error = true;
                        }
                    }

                    if bool_error {
                        match tx.send(()).await {
                            Ok(_) => break,
                            Err(e) => {
                                error!("{e}");
                                break;
                            }
                        }
                    }

                    if bool_mode {
                        match tx.send(()).await {
                            Ok(_) => (),
                            Err(e) => {
                                error!("{e}");
                                break;
                            }
                        }
                    }
                }
            });
        }
    }
}
