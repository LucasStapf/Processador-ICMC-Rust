use std::{fs::File, io::stdin};

use assembler::encoder::InstructionEncode;
use env_logger::{Builder, Target};
use isa::InstructionData;
use log::error;
use processor::Processor;

fn main() {
    init_logger();
    let mut p = Processor::default();

    let mut mem = Vec::new();
    mem.append(&mut InstructionData::LOADN { rx: 0, addr1: 3 }.encode());
    mem.append(&mut InstructionData::LOADN { rx: 1, addr1: 5 }.encode());
    mem.append(
        &mut InstructionData::ADD {
            rx: 2,
            ry: 1,
            rz: 0,
        }
        .encode(),
    );
    p.load_memory(&mem);

    let mut buf = String::new();

    loop {
        stdin().read_line(&mut buf);
        if let Err(e) = p.instruction_cicle() {
            error!("{e}");
        }
    }
}

fn init_logger() {
    let target = Box::new(File::create("log.txt").expect("Can't create file"));
    std::env::set_var("RUST_LOG", "debug");
    let mut builder = Builder::from_default_env();
    // builder.target(Target::Pipe(target));
    builder.target(Target::Stdout);
    builder.init();
}
