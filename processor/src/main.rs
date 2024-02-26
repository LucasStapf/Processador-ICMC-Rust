use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use env_logger::{Builder, Target};
use processor::Processor;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    // log config
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    // let p = Processor::default();
    // let run_signal = p.modules().control_unit.run_signal_send();
    //
    // let processor = Arc::new(Mutex::new(p));
    //
    // let t_processor = Processor::run(processor.clone());
    //
    // let t = thread::spawn(move || {
    //     log::debug!("Thread 2: Start...");
    //     thread::sleep(Duration::from_secs(5));
    //     log::debug!("Thread 2: Run processor...");
    //     run_signal
    //         .send_blocking(true)
    //         .expect("Falha ao enviar 'true'");
    //
    //     thread::sleep(Duration::from_secs(5));
    //     log::debug!("Thread 2: Stop processor...");
    //     run_signal
    //         .send_blocking(false)
    //         .expect("Falha ao enviar 'false'");
    //     thread::sleep(Duration::from_secs(5));
    // });
    //
    // t.join().unwrap();
    // log::debug!("Erro final: {:?}", t_processor.join().unwrap());
}
