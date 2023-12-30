use env_logger::{Builder, Target};
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::Application;
use processor::Processor;

mod instructions;
mod processor;
mod view;

const APP_ID: &str = "org.usp.ProcessadorIcmc";

fn main() -> glib::ExitCode {
    std::env::set_var("RUST_LOG", "simulator");

    // log config
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(crate::view::build_ui);
    app.run()
}
