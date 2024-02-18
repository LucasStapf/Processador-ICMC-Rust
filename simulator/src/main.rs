#![allow(dead_code, unused_imports)]

use std::thread;

use env_logger::{Builder, Target};

use adw::prelude::*;
use adw::{gio, glib, Application};
use gtk::gdk::Display;
use gtk::CssProvider;
use log::info;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use ui::{simulator_window, window};

mod files;
// mod mem_obj;
mod mem_row;
mod processor;
mod ui;

const APP_ID: &str = "org.ProcessadorICMC";
pub static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

fn main() -> glib::ExitCode {
    std::env::set_var("RUST_LOG", "debug");

    gio::resources_register_include!("compile.gresource")
        .expect("Falha ao carregar os recursos de UI.");
    // log config

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    info!("Iniciando o simulador");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../resources/sim.css"));

    let mem = CssProvider::new();
    mem.load_from_data(include_str!("../resources/row.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &mem,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let css = CssProvider::new();
    css.load_from_data(include_str!("../resources/entry-register.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // let window = ui::window::Window::new(app);

    let window = ui::simulator_window::SimulatorWindow::new(app);
    // let spinner = gtk::Spinner::new();
    // spinner.start();
    // let gtk_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // gtk_box.append(&spinner);
    // let window = gtk::ApplicationWindow::builder()
    //     .application(app)
    //     .default_height(400)
    //     .default_width(400)
    //     .child(&gtk_box)
    //     .build();

    window.present();
}
