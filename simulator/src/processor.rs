use std::{fmt::Display, usize};

use isa::Instruction;
use log::{debug, info};

use crate::instructions::InstructionCicle;

const PANIC_STR_MEM_INDEX: &str = "Acesso indevido da memória!";
const PANIC_STR_REG_INDEX: &str = "Acesso indevido dos registradores!";

const MEMORY_SIZE: usize = 32768;
const NUM_REGISTERS: usize = 8;

pub struct Processor {
    pub memory: Vec<usize>, // pub temp
    pub registers: [usize; NUM_REGISTERS],

    pub rx: usize,
    pub ry: usize,
    pub rz: usize,
    // Flag Register
    pub fr: [bool; 16],
    // Program Counter
    pub pc: usize,
    // Instruction Register
    pub ir: usize,
    // Stack Pointer
    pub sp: usize,
}

impl Processor {
    pub fn new() -> Self {
        info!(
            "Novo processador criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            MEMORY_SIZE, NUM_REGISTERS
        );

        let mut mem = Vec::with_capacity(MEMORY_SIZE);
        for _ in 0..MEMORY_SIZE {
            mem.push(0);
        }

        Self {
            memory: mem,
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

    pub fn mem(&self, index: usize) -> usize {
        *self.memory.get(index).expect(PANIC_STR_MEM_INDEX)
    }

    pub fn set_mem(&mut self, index: usize, value: usize) {
        *self.memory.get_mut(index).expect(PANIC_STR_MEM_INDEX) = value;
    }

    pub fn reg(&self, index: usize) -> usize {
        *self.registers.get(index).expect(PANIC_STR_REG_INDEX)
    }

    pub fn set_reg(&mut self, index: usize, value: usize) {
        *self.registers.get_mut(index).expect(PANIC_STR_REG_INDEX) = value;
    }

    fn search_cicle(&mut self) {
        self.ir = *self
            .memory
            .get(self.pc)
            .expect("Limite de memória do processador atingido!");
        debug!(
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
        debug!("Execution Cicle [{} {}]", instruction, instruction.mask());
        instruction.execution(self);
    }

    pub fn next(&mut self) {
        self.search_cicle();
        self.execution_cicle();
        debug!("{self}");
    }

    pub fn info(
        &self,
    ) -> (
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
    ) {
        (
            self.registers[0],
            self.registers[1],
            self.registers[2],
            self.registers[3],
            self.registers[4],
            self.registers[5],
            self.registers[6],
            self.registers[7],
            self.pc,
            self.sp,
            self.ir,
        )
    }

    pub fn run(&mut self) {
        loop {
            self.next()
        }
    }
}

impl Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Registers {:?}] [FR {:?}] [IR {}] [SP {}] [PC {}]",
            self.registers, self.fr, self.ir, self.sp, self.pc,
        )
    }
}
