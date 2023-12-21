use core::panic;

use crate::processor::Processor;
use isa::Instruction;

pub trait InstructionCicle {
    fn execution(&self, processor: &mut Processor);
}

impl InstructionCicle for Instruction {
    // add code here
    fn execution(&self, p: &mut Processor) {
        match self {
            Instruction::InvalidInstruction => panic!("Instrução inválida!"),
            Instruction::LOAD => {
                p.set_reg(p.rx, p.mem(p.mem(p.pc)));
                p.pc += 1;
            }
            Instruction::LOADN => {
                p.set_reg(p.rx, p.mem(p.pc));
                p.pc += 1;
            }
            Instruction::LOADI => todo!(),
            Instruction::STORE => todo!(),
            Instruction::STOREN => todo!(),
            Instruction::STOREI => todo!(),
            Instruction::MOV => todo!(),
            Instruction::INPUT => todo!(),
            Instruction::OUTPUT => todo!(),
            Instruction::OUTCHAR => todo!(),
            Instruction::INCHAR => todo!(),
            Instruction::SOUND => todo!(),
            Instruction::ADD => todo!(),
            Instruction::ADDC => todo!(),
            Instruction::SUB => todo!(),
            Instruction::SUBC => todo!(),
            Instruction::MUL => todo!(),
            Instruction::DIV => todo!(),
            Instruction::INC => todo!(),
            Instruction::DEC => todo!(),
            Instruction::MOD => todo!(),
            Instruction::AND => todo!(),
            Instruction::OR => todo!(),
            Instruction::XOR => todo!(),
            Instruction::NOT => todo!(),
            Instruction::SHIFTL0 => todo!(),
            Instruction::SHIFTL1 => todo!(),
            Instruction::SHIFTR0 => todo!(),
            Instruction::SHIFTR1 => todo!(),
            Instruction::ROTL => todo!(),
            Instruction::ROTR => todo!(),
            Instruction::CMP => todo!(),
            Instruction::JMP => todo!(),
            Instruction::JEQ => todo!(),
            Instruction::JNE => todo!(),
            Instruction::JZ => todo!(),
            Instruction::JNZ => todo!(),
            Instruction::JC => todo!(),
            Instruction::JNC => todo!(),
            Instruction::JGR => todo!(),
            Instruction::JLE => todo!(),
            Instruction::JEG => todo!(),
            Instruction::JEL => todo!(),
            Instruction::JOV => todo!(),
            Instruction::JNO => todo!(),
            Instruction::JDZ => todo!(),
            Instruction::JN => todo!(),
            Instruction::CALL => todo!(),
            Instruction::CEQ => todo!(),
            Instruction::CNE => todo!(),
            Instruction::CZ => todo!(),
            Instruction::CNZ => todo!(),
            Instruction::CC => todo!(),
            Instruction::CNC => todo!(),
            Instruction::CGR => todo!(),
            Instruction::CLE => todo!(),
            Instruction::CEG => todo!(),
            Instruction::CEL => todo!(),
            Instruction::COV => todo!(),
            Instruction::CNO => todo!(),
            Instruction::CDZ => todo!(),
            Instruction::CN => todo!(),
            Instruction::RTS => todo!(),
            Instruction::RTI => todo!(),
            Instruction::PUSH => todo!(),
            Instruction::POP => todo!(),
            Instruction::NOP => todo!(),
            Instruction::HALT => todo!(),
            Instruction::CLEARC => todo!(),
            Instruction::SETC => todo!(),
            Instruction::BREAKP => todo!(),
        }
    }
}
