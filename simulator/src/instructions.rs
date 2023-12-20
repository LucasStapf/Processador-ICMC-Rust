use crate::processor::Processor;
use isa::Instruction;

pub trait InstructionCicle {
    fn execution(&self, processor: &mut Processor);
}

impl InstructionCicle for Instruction {
    // add code here
    fn execution(&self, processor: &mut Processor) {
        match self {
            Instruction::INCHAR => todo!(),
            Instruction::LOAD => todo!(),
            Instruction::STORE => todo!(),
            Instruction::LOADIMED => todo!(),
            Instruction::STOREIMED => todo!(),
            Instruction::LOADINDEX => todo!(),
            Instruction::STOREINDEX => todo!(),
            Instruction::MOV => println!("Deu certo!"),
            Instruction::INPUT => todo!(),
            Instruction::OUTPUT => todo!(),
            Instruction::OUTCHAR => todo!(),
            Instruction::SOUND => todo!(),
            Instruction::ADD => todo!(),
            Instruction::SUB => todo!(),
            Instruction::MUL => todo!(),
            Instruction::DIV => todo!(),
            Instruction::INC => todo!(),
            Instruction::LMOD => todo!(),
            Instruction::LAND => todo!(),
            Instruction::LOR => todo!(),
            Instruction::LXOR => todo!(),
            Instruction::LNOT => todo!(),
            Instruction::SHIFT => todo!(),
            Instruction::CMP => todo!(),
            Instruction::BRA => todo!(),
            Instruction::JMP => todo!(),
            Instruction::CALL => todo!(),
            Instruction::RTS => todo!(),
            Instruction::RTI => todo!(),
            Instruction::PUSH => todo!(),
            Instruction::POP => todo!(),
            Instruction::CALLR => todo!(),
            Instruction::JMPR => todo!(),
            Instruction::NOP => todo!(),
            Instruction::HALT => todo!(),
            Instruction::CLEARC => todo!(),
            Instruction::BREAKP => todo!(),
        }
    }
}
