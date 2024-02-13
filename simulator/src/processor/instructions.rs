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
        let mut link = "https://lucasstapf.github.io/Processador-ICMC-Rust-Doc/isa/enum.Instruction.html#variant.".to_string();

        let mut link_string = |s: &str| -> String {
            link.push_str(s);
            format!("<a href=\"{}\"><b>{}</b></a>", link, s)
        };

        match self {
            isa::Instruction::InvalidInstruction => "Invalid Instruction".to_string(),

            isa::Instruction::LOAD => format!(
                "{} R{}, {}",
                link_string(&isa::Instruction::LOAD.to_string()),
                rx(processor.mem(addr).unwrap()),
                processor.mem(addr + 1).unwrap()
            ),

            isa::Instruction::LOADN => format!(
                "{} R{}, <i>#{}</i>",
                link_string(&isa::Instruction::LOADN.to_string()),
                rx(processor.mem(addr).unwrap()),
                processor.mem(addr + 1).unwrap()
            ),

            isa::Instruction::LOADI => format!(
                "{} R{}, R{}",
                link_string(&isa::Instruction::LOADI.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap())
            ),

            isa::Instruction::STORE => format!(
                "{} {}, R{}",
                link_string(&isa::Instruction::STORE.to_string()),
                processor.mem(addr + 1).unwrap(),
                rx(processor.mem(addr).unwrap())
            ),

            isa::Instruction::STOREN => {
                format!(
                    "{} {}, #{}",
                    link_string(&isa::Instruction::STOREN.to_string()),
                    processor.mem(addr + 1).unwrap(),
                    processor.mem(addr + 2).unwrap()
                )
            }

            isa::Instruction::STOREI => format!(
                "{} R{}, R{}",
                link_string(&isa::Instruction::STOREI.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap())
            ),

            isa::Instruction::MOV => match isa::bits(processor.mem(addr).unwrap(), 0..=1) {
                0 => format!(
                    "{} R{} R{}",
                    link_string(&isa::Instruction::MOV.to_string()),
                    rx(processor.mem(addr).unwrap()),
                    ry(processor.mem(addr).unwrap())
                ),
                1 => {
                    format!(
                        "{} R{} SP",
                        link_string(&isa::Instruction::MOV.to_string()),
                        rx(processor.mem(addr).unwrap()),
                    )
                }
                _ => {
                    format!(
                        "{} SP R{}",
                        link_string(&isa::Instruction::MOV.to_string()),
                        rx(processor.mem(addr).unwrap()),
                    )
                }
            },

            isa::Instruction::INPUT => todo!(),
            isa::Instruction::OUTPUT => todo!(),

            isa::Instruction::OUTCHAR => format!(
                "{} R{}, R{}",
                link_string(&isa::Instruction::OUTCHAR.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap())
            ),

            isa::Instruction::INCHAR => todo!(),
            isa::Instruction::SOUND => todo!(),

            isa::Instruction::ADD => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::ADD.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::ADDC => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::ADDC.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::SUB => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::SUB.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::SUBC => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::SUBC.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::MUL => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::MUL.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::DIV => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::DIV.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::INC => format!(
                "{} R{}",
                link_string(&isa::Instruction::INC.to_string()),
                rx(processor.mem(addr).unwrap()),
            ),

            isa::Instruction::DEC => format!(
                "{} R{}",
                link_string(&isa::Instruction::DEC.to_string()),
                rx(processor.mem(addr).unwrap()),
            ),

            isa::Instruction::MOD => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::MOD.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::AND => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::AND.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::OR => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::OR.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::XOR => format!(
                "{} R{}, R{}, R{}",
                link_string(&isa::Instruction::XOR.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
                rz(processor.mem(addr).unwrap())
            ),

            isa::Instruction::NOT => format!(
                "{} R{}, R{}",
                link_string(&isa::Instruction::NOT.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
            ),

            isa::Instruction::SHIFTL0 => format!(
                "{} R{}, {}",
                link_string(&isa::Instruction::SHIFTR0.to_string()),
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::SHIFTL1 => format!(
                "{} R{}, {}",
                link_string(&isa::Instruction::SHIFTL1.to_string()),
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::SHIFTR0 => format!(
                "{} R{}, {}",
                link_string(&isa::Instruction::SHIFTR0.to_string()),
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::SHIFTR1 => format!(
                "{} R{}, {}",
                link_string(&isa::Instruction::SHIFTR1.to_string()),
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::ROTL => format!(
                "{} R{}, {}",
                link_string(&isa::Instruction::ROTL.to_string()),
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::ROTR => format!(
                "{} R{}, {}",
                link_string(&isa::Instruction::ROTR.to_string()),
                rx(processor.mem(addr).unwrap()),
                isa::bits(processor.mem(addr).unwrap(), 0..=3)
            ),

            isa::Instruction::CMP => format!(
                "{} R{}, R{}",
                link_string(&isa::Instruction::CMP.to_string()),
                rx(processor.mem(addr).unwrap()),
                ry(processor.mem(addr).unwrap()),
            ),

            isa::Instruction::JMP => format!(
                "{} {}",
                link_string(&isa::Instruction::JMP.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JEQ => format!(
                "{} {}",
                link_string(&isa::Instruction::JEQ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JNE => format!(
                "{} {}",
                link_string(&isa::Instruction::JNE.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JZ => format!(
                "{} {}",
                link_string(&isa::Instruction::JZ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JNZ => format!(
                "{} {}",
                link_string(&isa::Instruction::JNZ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JC => format!(
                "{} {}",
                link_string(&isa::Instruction::JC.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JNC => format!(
                "{} {}",
                link_string(&isa::Instruction::JNC.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JGR => format!(
                "{} {}",
                link_string(&isa::Instruction::JGR.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JLE => format!(
                "{} {}",
                link_string(&isa::Instruction::JLE.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JEG => format!(
                "{} {}",
                link_string(&isa::Instruction::JEG.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JEL => format!(
                "{} {}",
                link_string(&isa::Instruction::JEL.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JOV => format!(
                "{} {}",
                link_string(&isa::Instruction::JOV.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JNO => format!(
                "{} {}",
                link_string(&isa::Instruction::JNO.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JDZ => format!(
                "{} {}",
                link_string(&isa::Instruction::JDZ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::JN => format!(
                "{} {}",
                link_string(&isa::Instruction::JN.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),

            isa::Instruction::CALL => format!(
                "{} {}",
                link_string(&isa::Instruction::CALL.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CEQ => format!(
                "{} {}",
                link_string(&isa::Instruction::CEQ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CNE => format!(
                "{} {}",
                link_string(&isa::Instruction::CNE.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CZ => format!(
                "{} {}",
                link_string(&isa::Instruction::CZ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CNZ => format!(
                "{} {}",
                link_string(&isa::Instruction::CNZ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CC => format!(
                "{} {}",
                link_string(&isa::Instruction::CC.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CNC => format!(
                "{} {}",
                link_string(&isa::Instruction::CNC.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CGR => format!(
                "{} {}",
                link_string(&isa::Instruction::CGR.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CLE => format!(
                "{} {}",
                link_string(&isa::Instruction::CLE.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CEG => format!(
                "{} {}",
                link_string(&isa::Instruction::CEG.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CEL => format!(
                "{} {}",
                link_string(&isa::Instruction::CEL.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::COV => format!(
                "{} {}",
                link_string(&isa::Instruction::COV.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CNO => format!(
                "{} {}",
                link_string(&isa::Instruction::CNO.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CDZ => format!(
                "{} {}",
                link_string(&isa::Instruction::CDZ.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),
            isa::Instruction::CN => format!(
                "{} {}",
                link_string(&isa::Instruction::CN.to_string()),
                processor.mem(addr + 1).unwrap(),
            ),

            isa::Instruction::RTS => format!("{}", link_string(&isa::Instruction::RTS.to_string())),
            isa::Instruction::RTI => format!("{}", link_string(&isa::Instruction::RTI.to_string())),

            isa::Instruction::PUSH => match isa::bits(processor.mem(addr).unwrap(), 6..=6) {
                0 => format!(
                    "{} R{}",
                    link_string(&isa::Instruction::PUSH.to_string()),
                    rx(processor.mem(addr).unwrap())
                ),
                _ => format!("{} FR", link_string(&isa::Instruction::PUSH.to_string())),
            },

            isa::Instruction::POP => match isa::bits(processor.mem(addr).unwrap(), 6..=6) {
                0 => format!(
                    "{} R{}",
                    link_string(&isa::Instruction::POP.to_string()),
                    rx(processor.mem(addr).unwrap())
                ),
                _ => format!("{} FR", link_string(&isa::Instruction::POP.to_string())),
            },

            isa::Instruction::NOP => format!("{}", link_string(&isa::Instruction::NOP.to_string())),

            isa::Instruction::HALT => {
                format!("{}", link_string(&isa::Instruction::HALT.to_string()))
            }

            isa::Instruction::CLEARC => {
                format!("{}", link_string(&isa::Instruction::CLEARC.to_string()))
            }

            isa::Instruction::SETC => {
                format!("{}", link_string(&isa::Instruction::SETC.to_string()))
            }

            isa::Instruction::BREAKP => {
                format!("{}", link_string(&isa::Instruction::BREAKP.to_string()))
            }
        }
    }
}
