use isa::MemoryCell;
use processor::Processor;

pub trait InstructionDisplay {
    fn display(&self, addr: MemoryCell, processor: &Processor) -> String;
}

fn rx(mem: MemoryCell) -> MemoryCell {
    isa::bits(mem, 7..=9)
}

fn ry(mem: MemoryCell) -> MemoryCell {
    isa::bits(mem, 4..=6)
}

fn rz(mem: MemoryCell) -> MemoryCell {
    isa::bits(mem, 1..=3)
}

impl InstructionDisplay for isa::Instruction {
    fn display(&self, addr: MemoryCell, processor: &Processor) -> String {
        match self {
            isa::Instruction::InvalidInstruction => "Invalid Instruction".to_string(),

            isa::Instruction::LOAD => format!(
                "LOAD R{}, {}",
                rx(processor.mem(addr).unwrap()),
                processor.mem(addr + 1).unwrap()
            ),

            isa::Instruction::LOADN => format!(
                "LOADN R{}, #{}",
                rx(processor.mem(addr).unwrap()),
                processor.mem(addr + 1).unwrap()
            ),

            isa::Instruction::LOADI => format!(
                "LOADI R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap())
            ),

            isa::Instruction::STORE => format!(
                "STORE {}, R{}",
                processor.mem(addr + 1).unwrap(),
                rx(processor.mem(addr).unwrap())
            ),

            isa::Instruction::STOREN => format!(
                "STOREN {}, #{}",
                processor.mem(addr + 1).unwrap(),
                processor.mem(addr + 2).unwrap()
            ),

            isa::Instruction::STOREI => format!(
                "STOREI R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap())
            ),

            isa::Instruction::MOV => todo!(),
            isa::Instruction::INPUT => todo!(),
            isa::Instruction::OUTPUT => todo!(),
            isa::Instruction::OUTCHAR => todo!(),
            isa::Instruction::INCHAR => todo!(),
            isa::Instruction::SOUND => todo!(),

            isa::Instruction::ADD => format!(
                "ADD R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::ADDC => format!(
                "ADDC R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::SUB => format!(
                "SUB R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::SUBC => format!(
                "SUBC R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::MUL => format!(
                "MUL R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::DIV => format!(
                "DIV R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::INC => format!("INC R{}", rx(processor.mem(addr).unwrap()),),

            isa::Instruction::DEC => format!("DEC R{}", rx(processor.mem(addr).unwrap()),),

            isa::Instruction::MOD => format!(
                "MOD R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::AND => format!(
                "AND R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::OR => format!(
                "OR R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::XOR => format!(
                "XOR R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::NOT => todo!(),
            isa::Instruction::SHIFTL0 => todo!(),
            isa::Instruction::SHIFTL1 => todo!(),
            isa::Instruction::SHIFTR0 => todo!(),
            isa::Instruction::SHIFTR1 => todo!(),
            isa::Instruction::ROTL => todo!(),
            isa::Instruction::ROTR => todo!(),
            isa::Instruction::CMP => todo!(),
            isa::Instruction::JMP => todo!(),
            isa::Instruction::JEQ => todo!(),
            isa::Instruction::JNE => todo!(),
            isa::Instruction::JZ => todo!(),
            isa::Instruction::JNZ => todo!(),
            isa::Instruction::JC => todo!(),
            isa::Instruction::JNC => todo!(),
            isa::Instruction::JGR => todo!(),
            isa::Instruction::JLE => todo!(),
            isa::Instruction::JEG => todo!(),
            isa::Instruction::JEL => todo!(),
            isa::Instruction::JOV => todo!(),
            isa::Instruction::JNO => todo!(),
            isa::Instruction::JDZ => todo!(),
            isa::Instruction::JN => todo!(),
            isa::Instruction::CALL => todo!(),
            isa::Instruction::CEQ => todo!(),
            isa::Instruction::CNE => todo!(),
            isa::Instruction::CZ => todo!(),
            isa::Instruction::CNZ => todo!(),
            isa::Instruction::CC => todo!(),
            isa::Instruction::CNC => todo!(),
            isa::Instruction::CGR => todo!(),
            isa::Instruction::CLE => todo!(),
            isa::Instruction::CEG => todo!(),
            isa::Instruction::CEL => todo!(),
            isa::Instruction::COV => todo!(),
            isa::Instruction::CNO => todo!(),
            isa::Instruction::CDZ => todo!(),
            isa::Instruction::CN => todo!(),
            isa::Instruction::RTS => todo!(),
            isa::Instruction::RTI => todo!(),
            isa::Instruction::PUSH => todo!(),
            isa::Instruction::POP => todo!(),
            isa::Instruction::NOP => "NOP".to_string(),
            isa::Instruction::HALT => todo!(),
            isa::Instruction::CLEARC => todo!(),
            isa::Instruction::SETC => todo!(),
            isa::Instruction::BREAKP => todo!(),
        }
    }
}
