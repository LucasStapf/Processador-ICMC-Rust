use std::fmt::Display;

use gtk4::glib::clone;
use log::error;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use processor::errors::ProcError;
use processor::Processor;

use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::{self, gdk, glib};

const MAIN_SCREEN_TITLE: &str = "Processador ICMC";

static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RunMode {
    Automatic,
    StepByStep(bool),
}

fn run_mode(processor: &mut Processor, mode: &mut RunMode) -> std::result::Result<(), ProcError> {
    match mode {
        RunMode::Automatic => processor.next(),
        RunMode::StepByStep(b) => {
            if *b {
                *b = false;
                processor.next()
            } else {
                Ok(())
            }
        }
    }
}

impl Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunMode::Automatic => write!(f, "Automatic"),
            RunMode::StepByStep(_) => write!(f, "StepByStep"),
        }
    }
}

pub fn build_ui(app: &Application) {
    let (tx, rx) = async_channel::bounded(1);
    let receiver = run_processor(rx);

    let el_r0 = gtk4::Label::new(None);
    let el_r1 = gtk4::Label::new(None);
    let el_r2 = gtk4::Label::new(None);
    let el_r3 = gtk4::Label::new(None);
    let el_r4 = gtk4::Label::new(None);
    let el_r5 = gtk4::Label::new(None);
    let el_r6 = gtk4::Label::new(None);
    let el_r7 = gtk4::Label::new(None);
    let el_pc = gtk4::Label::new(None);
    let el_sp = gtk4::Label::new(None);
    let el_ir = gtk4::Label::new(None);

    glib::spawn_future_local(
        clone!(@weak el_r0, @weak el_r1, @weak el_r2, @weak el_r3, @weak el_r4, @weak el_r5, @weak el_r6, @weak el_r7, @weak el_pc, @weak el_sp, @weak el_ir => async move {
            loop {
                match receiver.recv().await {
                    Ok(info) => {
                        el_r0.set_text(&info.0.to_string());
                        el_r1.set_text(&info.1.to_string());
                        el_r2.set_text(&info.2.to_string());
                        el_r3.set_text(&info.3.to_string());
                        el_r4.set_text(&info.4.to_string());
                        el_r5.set_text(&info.5.to_string());
                        el_r6.set_text(&info.6.to_string());
                        el_r7.set_text(&info.7.to_string());
                        el_pc.set_text(&info.8.to_string());
                        el_sp.set_text(&info.9.to_string());
                        el_ir.set_text(&info.10.to_string());
                    }
                    Err(e) => {
                        error!("[Receiving info] {e}");
                        break;
                    },
                }
            }
        }),
    );

    let h_boxex = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();

    h_boxex.append(&el_r0);
    h_boxex.append(&el_r1);
    h_boxex.append(&el_r2);
    h_boxex.append(&el_r3);
    h_boxex.append(&el_r4);
    h_boxex.append(&el_r5);
    h_boxex.append(&el_r6);
    h_boxex.append(&el_r7);
    h_boxex.append(&el_pc);
    h_boxex.append(&el_sp);
    h_boxex.append(&el_ir);

    let h_pane = gtk4::Paned::builder()
        .start_child(&h_boxex)
        .orientation(gtk4::Orientation::Horizontal)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(MAIN_SCREEN_TITLE)
        .child(&h_pane)
        .build();

    let event_controller = gtk4::EventControllerKey::new();

    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Home => {
                let _ = tx
                    .send_blocking(RunMode::Automatic)
                    .map_err(|e| error!("[Sending RunMode] {e}"));
            }
            gdk::Key::Page_Up => {
                let _ = tx
                    .send_blocking(RunMode::StepByStep(true))
                    .map_err(|e| error!("[Sending RunMode] {e}"));
            }
            gdk::Key::Page_Down => {
                let _ = tx
                    .send_blocking(RunMode::StepByStep(false))
                    .map_err(|e| error!("[Sending RunMode] {e}"));
            }
            _ => (),
        }
        glib::Propagation::Proceed
    });

    window.add_controller(event_controller);
    window.present();
}

fn run_processor(
    rx: async_channel::Receiver<RunMode>,
) -> async_channel::Receiver<(
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
)> {
    let mut p = Processor::new();
    let (t, r) = async_channel::bounded(1);

    RUNTIME.spawn(async move {
        let mut mode = RunMode::StepByStep(false);
        loop {
            match rx.try_recv() {
                Ok(m) => {
                    mode = m;
                    if let Err(e) = run_mode(&mut p, &mut mode) {
                        error!("{e}");
                        break;
                    }
                    if let Err(e) = t.send(p.state()).await {
                        error!("[Sending info] {e}");
                        break;
                    }
                }
                Err(e) => match e {
                    async_channel::TryRecvError::Empty => {
                        if let Err(e) = run_mode(&mut p, &mut mode) {
                            error!("{e}");
                            break;
                        }
                        if let Err(e) = t.send(p.state()).await {
                            error!("[Sending info] {e}");
                            break;
                        }
                    }
                    async_channel::TryRecvError::Closed => {
                        error!("[Sending info] {e}");
                        break;
                    }
                },
            }
        }
    });

    r
}
