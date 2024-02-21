pub mod instructions;

use adw::glib;
use async_channel::{Receiver, Sender};
use cairo::glib::clone;
use log::{debug, error};
use once_cell::sync::Lazy;
use processor::{errors::ProcessorError, Processor};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;

use crate::{ui::window::InfoType, RUNTIME};

// pub static RUNTIME: Lazy<Runtime> =
// Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

#[derive(Clone)]
pub enum RunMode {
    Run,
    Debug,
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Debug
    }
}

#[derive(Clone)]
pub struct ProcessorManager {
    pub mode: Arc<Mutex<RunMode>>,
    pub processor: Arc<Mutex<Processor>>,
    pub error: Arc<Mutex<Option<ProcessorError>>>,
    pub tx: Option<Sender<Option<ProcessorError>>>,
    pub rx: Option<Receiver<bool>>,

    pub mode_notification_recv: Receiver<()>,
    pub mode_notification_send: Sender<()>,
    pub status_notification_recv: Receiver<Option<ProcessorError>>,
    pub status_notification_send: Sender<Option<ProcessorError>>,
}

impl Default for ProcessorManager {
    fn default() -> Self {
        let (status_notification_send, status_notification_recv) = async_channel::bounded(1);
        let (mode_notification_send, mode_notification_recv) = async_channel::bounded(1);
        Self {
            mode: Default::default(),
            processor: Default::default(),
            error: Arc::new(Mutex::new(None)),
            tx: None,
            rx: None,
            status_notification_recv,
            status_notification_send,
            mode_notification_recv,
            mode_notification_send,
        }
    }
}

impl ProcessorManager {
    pub fn new(tx: Option<Sender<Option<ProcessorError>>>, rx: Option<Receiver<bool>>) -> Self {
        Self {
            tx,
            rx,
            ..Default::default()
        }
    }

    fn next_instruction_cicle(p: &Mutex<Processor>) -> Result<(), ProcessorError> {
        let mut processor = p.lock().map_err(|_| ProcessorError::Generic {
            title: "Poison Error".to_string(),
            description: "Falha ao acessar o processador!".to_string(),
        })?;

        match processor.instruction_cicle() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // pub fn run(&self, tx: Sender<InfoType<ProcessorError>>) {
    //     let m = self.mode.clone();
    //     let p = self.processor.clone();
    //
    //     if let Ok(mut p) = p.lock() {
    //         p.set_mem(4, 0b1110010000000000);
    //         p.set_mem(5, 0b0000110000000000);
    //         p.set_mem(7, 0b1011110000000000);
    //     }
    //
    //     RUNTIME.spawn(async move {
    //         loop {
    //             let mode_test;
    //             match m.lock() {
    //                 Ok(mut mode) => match mode.as_ref() {
    //                     Some(m) => match m {
    //                         RunMode::Run => mode_test = RunMode::Run,
    //                         RunMode::Debug => {
    //                             *mode = None;
    //                             mode_test = RunMode::Debug;
    //                         }
    //                     },
    //                     None => continue,
    //                 },
    //                 Err(e) => {
    //                     error!("{e}");
    //                     tx.send_blocking(InfoType::Error(ProcessorError::Generic {
    //                         title: "Erro inesperado".to_string(),
    //                         description: e.to_string(),
    //                     }))
    //                     .expect("Falha ao enviar o erro!");
    //                     break;
    //                 }
    //             }
    //
    //             let pixel_test;
    //             match p.lock() {
    //                 Ok(mut p) => match p.next() {
    //                     Ok(_) => pixel_test = p.pixel(),
    //                     Err(e) => {
    //                         error!("{e}");
    //                         tx.send_blocking(InfoType::Error(e))
    //                             .expect("Falha ao enviar o erro!");
    //                         break;
    //                     }
    //                 },
    //                 Err(e) => {
    //                     error!("{e}");
    //                     tx.send_blocking(InfoType::Error(ProcessorError::Generic {
    //                         title: "Erro inesperado".to_string(),
    //                         description: e.to_string(),
    //                     }))
    //                     .expect("Falha ao enviar o erro!");
    //                     break;
    //                 }
    //             }
    //
    //             match mode_test {
    //                 RunMode::Run => (),
    //                 RunMode::Debug => tx
    //                     .send(InfoType::UpdateUI)
    //                     .await
    //                     .expect("Falha ao enviar mensagem UpdateUI"),
    //             }
    //
    //             match pixel_test {
    //                 Some((p, i)) => tx
    //                     .send(InfoType::UpdateScreen(p, i))
    //                     .await
    //                     .expect("Falha ao enviar mensagem UpdateUI"),
    //                 None => (),
    //             }
    //         }
    //     });
    // }

    // pub fn run(&self) {
    //     if let Some(tx) = self.tx.clone() {
    //         let mode = self.mode.clone();
    //         let processor = self.processor.clone();
    //         RUNTIME.spawn(async move {
    //             loop {
    //                 let mut error: Option<ProcError> = None;
    //                 let mut bool_mode = false;
    //                 let mut bool_error = false;
    //                 match mode.lock() {
    //                     Ok(mut m) => match *m {
    //                         RunMode::Run => (),
    //                         RunMode::Debug(b) => match b {
    //                             true => {
    //                                 *m = RunMode::Debug(false);
    //                                 bool_mode = true;
    //                             }
    //                             false => continue,
    //                         },
    //                     },
    //                     Err(e) => {
    //                         error!("{e}");
    //                         error = Some(ProcError::ProcessorPanic);
    //                         bool_error = true;
    //                     }
    //                 }
    //
    //                 if bool_error {
    //                     match tx.send(error).await {
    //                         Ok(_) => break,
    //                         Err(e) => {
    //                             error!("{e}");
    //                             break;
    //                         }
    //                     }
    //                 }
    //
    //                 match processor.lock() {
    //                     Ok(mut p) => match p.next() {
    //                         Ok(b) => {
    //                             if b == false {
    //                                 *mode.lock().unwrap() = RunMode::Debug(false);
    //                                 bool_mode = true;
    //                             }
    //                             // p.set_mem(4, 0b1011111011000000).unwrap();
    //                             p.set_mem(2, 0b1100001111000000).unwrap();
    //                             p.set_mem(5, 0b1110011111000000).unwrap();
    //                             p.set_mem(6, 0xA2).unwrap();
    //                             p.set_mem(10000, 0b0011100000000000).unwrap();
    //                         } // TIRAR DPS SO TEST
    //                         Err(e) => {
    //                             error!("{e}");
    //                             error = Some(e);
    //                             bool_error = true;
    //                         }
    //                     },
    //                     Err(e) => {
    //                         error!("{e}");
    //                         bool_error = true;
    //                     }
    //                 }
    //
    //                 if bool_error {
    //                     match tx.send(error).await {
    //                         Ok(_) => break,
    //                         Err(e) => {
    //                             error!("{e}");
    //                             break;
    //                         }
    //                     }
    //                 }
    //
    //                 if bool_mode {
    //                     match tx.send(error).await {
    //                         Ok(_) => (),
    //                         Err(e) => {
    //                             error!("{e}");
    //                             break;
    //                         }
    //                     }
    //                 }
    //             }
    //         });
    //     }
    // }
}
