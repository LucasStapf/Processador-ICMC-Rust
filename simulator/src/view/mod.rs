use std::fmt::Display;

use gtk4::glib::clone;
use gtk4::Label;
use log::error;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use crate::processor::Processor;

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

fn run_mode(processor: &mut Processor, mode: &mut RunMode) {
    match mode {
        RunMode::Automatic => processor.next(),
        RunMode::StepByStep(b) => {
            if *b {
                processor.next();
                *b = false;
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

    let l_r0 = Label::new(None);
    let l_r1 = Label::new(None);
    let l_r2 = Label::new(None);
    let l_r3 = Label::new(None);
    let l_r4 = Label::new(None);
    let l_r5 = Label::new(None);
    let l_r6 = Label::new(None);
    let l_r7 = Label::new(None);
    let l_pc = Label::new(None);
    let l_sp = Label::new(None);
    let l_ir = Label::new(None);

    glib::spawn_future_local(
        clone!(@weak l_r0, @weak l_r1, @weak l_r2, @weak l_r3, @weak l_r4, @weak l_r5, @weak l_r6, @weak l_r7, @weak l_pc, @weak l_sp, @weak l_ir => async move {
            loop {
                match receiver.recv().await {
                    Ok(info) => {
                        l_r0.set_text(&info.0.to_string());
                        l_r1.set_text(&info.1.to_string());
                        l_r2.set_text(&info.2.to_string());
                        l_r3.set_text(&info.3.to_string());
                        l_r4.set_text(&info.4.to_string());
                        l_r5.set_text(&info.5.to_string());
                        l_r6.set_text(&info.6.to_string());
                        l_r7.set_text(&info.7.to_string());
                        l_pc.set_text(&info.8.to_string());
                        l_sp.set_text(&info.9.to_string());
                        l_ir.set_text(&info.10.to_string());
                    }
                    Err(e) => {
                        error!("{e}");
                        break;
                    },
                }
            }
        }),
    );

    let h_boxex = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();

    h_boxex.append(&l_r0);
    h_boxex.append(&l_r1);
    h_boxex.append(&l_r2);
    h_boxex.append(&l_r3);
    h_boxex.append(&l_r4);
    h_boxex.append(&l_r5);
    h_boxex.append(&l_r6);
    h_boxex.append(&l_r7);

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
                tx.send(RunMode::Automatic);
            }
            gdk::Key::Page_Up => {
                tx.send(RunMode::StepByStep(true));
            }
            gdk::Key::Page_Down => {
                tx.send(RunMode::StepByStep(false));
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
                    run_mode(&mut p, &mut mode);
                    if let Err(e) = t.send(p.info()).await {
                        error!("{e}");
                        break;
                    }
                }
                Err(e) => match e {
                    async_channel::TryRecvError::Empty => {
                        run_mode(&mut p, &mut mode);
                        if let Err(e) = t.send(p.info()).await {
                            error!("{e}");
                            break;
                        }
                    }
                    async_channel::TryRecvError::Closed => {
                        error!("{e}");
                        break;
                    }
                },
            }
        }
    });

    r
}
