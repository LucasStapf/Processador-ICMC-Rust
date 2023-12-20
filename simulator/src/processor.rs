use isa::Instruction;

use crate::instructions::InstructionCicle;

const MEMORY_SIZE: usize = 32768;
const NUM_REGISTERS: usize = 8;

pub struct Processor {
    memory: [u16; MEMORY_SIZE],
    registers: [usize; NUM_REGISTERS],

    // Flag Register
    fr: [bool; 16],
    // Program Counter
    pc: usize,
    // Instruction Register
    ir: usize,
    // Stack Pointer
    sp: usize,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            registers: [0; NUM_REGISTERS],
            fr: [false; 16],
            pc: 0,
            ir: 0,
            sp: 0,
        }
    }

    pub fn begin_cicle(&mut self) {
        let inst = Instruction::MOV;
        inst.execution(self);
    }
}
