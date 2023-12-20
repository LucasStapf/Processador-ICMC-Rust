use env_logger::{builder, Builder, Target};
use processor::Processor;

mod instructions;
mod processor;

fn main() {
    std::env::set_var("RUST_LOG", "simulator");

    // log config
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    let mut processor = Processor::new();
    processor.begin_cicle();
}
