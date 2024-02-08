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
                "<b>LOAD</b> R{}, {}",
                rx(processor.mem(addr).unwrap()),
                processor.mem(addr + 1).unwrap()
            ),

            isa::Instruction::LOADN => format!(
                "<b>LOADN</b> R{}, <i>#{}</i>",
                rx(processor.mem(addr).unwrap()),
                processor.mem(addr + 1).unwrap()
            ),

            isa::Instruction::LOADI => format!(
                "<b>LOADI</b> R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap())
            ),

            isa::Instruction::STORE => format!(
                "<b>STORE</b> {}, R{}",
                processor.mem(addr + 1).unwrap(),
                rx(processor.mem(addr).unwrap())
            ),

            isa::Instruction::STOREN => format!(
                "<b>STOREN</b> {}, #{}",
                processor.mem(addr + 1).unwrap(),
                processor.mem(addr + 2).unwrap()
            ),

            isa::Instruction::STOREI => format!(
                "<b>STOREI</b> R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap())
            ),

            isa::Instruction::MOV => match isa::bits(processor.mem(addr).unwrap(), 0..=1) {
                0 => format!(
                    "<b>MOV</b> R{} R{}",
                    rx(processor.mem(addr).unwrap()),
                    ry(processor.mem(addr).unwrap())
                ),
                1 => format!("<b>MOV</b> R{} SP", rx(processor.mem(addr).unwrap()),),
                _ => format!("<b>MOV</b> SP R{}", rx(processor.mem(addr).unwrap()),),
            },

            isa::Instruction::INPUT => todo!(),
            isa::Instruction::OUTPUT => todo!(),
            isa::Instruction::OUTCHAR => todo!(),
            isa::Instruction::INCHAR => todo!(),
            isa::Instruction::SOUND => todo!(),

            isa::Instruction::ADD => format!(
                "<b>ADD</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::ADDC => format!(
                "<b>ADDC</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::SUB => format!(
                "<b>SUB</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::SUBC => format!(
                "<b>SUBC</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::MUL => format!(
                "<b>MUL</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::DIV => format!(
                "<b>DIV</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::INC => format!("<b>INC</b> R{}", rx(processor.mem(addr).unwrap()),),

            isa::Instruction::DEC => format!("<b>DEC</b> R{}", rx(processor.mem(addr).unwrap()),),

            isa::Instruction::MOD => format!(
                "<b>MOD</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::AND => format!(
                "<b>AND</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::OR => format!(
                "<b>OR</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::XOR => format!(
                "<b>XOR</b> R{}, R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::NOT => format!(
                "<b>NOT</b> R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
            ),

            isa::Instruction::SHIFTL0 => format!(
                "<b>SHIFTL0</b> R{}, {}",
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::SHIFTL1 => format!(
                "<b>SHIFTL1</b> R{}, {}",
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::SHIFTR0 => format!(
                "<b>SHIFTR0</b> R{}, {}",
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::SHIFTR1 => format!(
                "<b>SHIFTR1</b> R{}, {}",
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::ROTL => format!(
                "<b>ROTL</b> R{}, {}",
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::ROTR => format!(
                "<b>ROTR</b> R{}, {}",
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::CMP => format!(
                "<b>CMP</b> R{}, R{}",
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
            ),

            isa::Instruction::JMP => format!("<b>JMP</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JEQ => format!("<b>JEQ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JNE => format!("<b>JNE</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JZ => format!("<b>JZ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JNZ => format!("<b>JNZ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JC => format!("<b>JC</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JNC => format!("<b>JNC</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JGR => format!("<b>JGR</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JLE => format!("<b>JLE</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JEG => format!("<b>JEG</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JEL => format!("<b>JEL</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JOV => format!("<b>JOV</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JNO => format!("<b>JNO</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JDZ => format!("<b>JDZ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::JN => format!("<b>JN</b> {}", processor.mem(addr + 1).unwrap(),),

            isa::Instruction::CALL => format!("<b>CALL</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CEQ => format!("<b>CEQ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CNE => format!("<b>CNE</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CZ => format!("<b>CZ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CNZ => format!("<b>CNZ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CC => format!("<b>CC</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CNC => format!("<b>CNC</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CGR => format!("<b>CGR</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CLE => format!("<b>CLE</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CEG => format!("<b>CEG</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CEL => format!("<b>CEL</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::COV => format!("<b>COV</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CNO => format!("<b>CNO</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CDZ => format!("<b>CDZ</b> {}", processor.mem(addr + 1).unwrap(),),
            isa::Instruction::CN => format!("<b>CN</b> {}", processor.mem(addr + 1).unwrap(),),

            isa::Instruction::RTS => format!("<b>RTS</b>"),
            isa::Instruction::RTI => format!("<b>RTI</b>"),

            isa::Instruction::PUSH => match isa::bits(processor.mem(addr).unwrap(), 6..=6) {
                0 => format!("<b>PUSH</b> R{}", rx(processor.mem(addr).unwrap())),
                _ => format!("<b>PUSH</b> FR"),
            },

            isa::Instruction::POP => match isa::bits(processor.mem(addr).unwrap(), 6..=6) {
                0 => format!("<b>POP</b> R{}", rx(processor.mem(addr).unwrap())),
                _ => format!("<b>POP</b> FR"),
            },

            isa::Instruction::NOP => format!("<b>NOP</b>"),
            isa::Instruction::HALT => format!("<b>HALT</b>"),
            isa::Instruction::CLEARC => format!("<b>CLEARC</b>"),
            isa::Instruction::SETC => format!("<b>SETC</b>"),
            isa::Instruction::BREAKP => format!("<b>BREAKP</b>"),
        }
    }
}
