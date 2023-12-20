use core::panic;

use isa::Instruction;
use log::info;

use crate::instructions::InstructionCicle;

const MEMORY_SIZE: usize = 32768;
const NUM_REGISTERS: usize = 8;

pub struct Processor {
    memory: [usize; MEMORY_SIZE],
    registers: [usize; NUM_REGISTERS],

    rx: usize,
    ry: usize,
    rz: usize,
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
            rx: 0,
            ry: 0,
            rz: 0,
            fr: [false; 16],
            pc: 0,
            ir: 0,
            sp: 0,
        }
    }

    fn search_cicle(&mut self) {
        self.ir = *self
            .memory
            .get(self.pc)
            .expect("Limite de memória do processador atingido!");
        info!(
            "Search Cicle [Instruction Register {:016b}] [Program Counter {}]",
            self.ir, self.pc
        );
        self.pc += 1;
    }

    fn execution_cicle(&mut self) {
        self.rx = isa::bits(self.ir, 7..=9);
        self.ry = isa::bits(self.ir, 4..=6);
        self.rz = isa::bits(self.ir, 1..=3);

        let instruction = Instruction::get_instruction(self.ir);
        info!("Execution Cicle [{}]", instruction);
        instruction.execution(self);
    }

    pub fn begin_cicle(&mut self) {
        self.search_cicle();
        self.execution_cicle();
    }
}
