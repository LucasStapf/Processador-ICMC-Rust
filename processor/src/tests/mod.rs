use isa::Instruction;

use crate::{instructions::InstructionCicle, Processor};

mod arithmetic;

pub fn setup_processor() -> Processor {
    let p = Processor::with_capacity(10);
    p
}

pub fn exec(instr: Instruction, processor: &mut Processor) {
    instr.execution(processor).unwrap();
}
