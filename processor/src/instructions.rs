use crate::{FlagIndex, MAX_VALUE_MEMORY};

use super::{ProcError, Processor};
use isa::Instruction;

pub trait InstructionCicle {
    fn execution(&self, processor: &mut Processor) -> Result<(), ProcError>;
}

impl InstructionCicle for Instruction {
    fn execution(&self, p: &mut Processor) -> Result<(), ProcError> {
        match self {
            Instruction::InvalidInstruction => return Err(ProcError::InvalidInstruction(p.ir())),
            Instruction::LOAD => {
                p.set_reg(p.rx(), p.mem(p.mem(p.pc())?)?)?;
                p.inc_pc(1)?;
            }
            Instruction::LOADN => {
                p.set_reg(p.rx(), p.mem(p.pc())?)?;
                p.inc_pc(1)?;
            }
            Instruction::LOADI => {
                p.set_reg(p.rx(), p.reg(p.ry())?)?;
            }
            Instruction::STORE => {
                p.set_mem(p.mem(p.pc())?, p.reg(p.rx())?)?;
                p.inc_pc(1)?;
            }
            Instruction::STOREN => {
                p.set_mem(p.mem(p.pc())?, p.mem(p.pc() + 1)?)?;
                p.inc_pc(2)?;
            }
            Instruction::STOREI => {
                p.set_mem(p.reg(p.rx())?, p.reg(p.ry())?)?;
            }
            Instruction::MOV => match isa::bits(p.ir(), 0..=1) {
                0 => p.set_reg(p.rx(), p.reg(p.ry())?)?,
                1 => p.set_reg(p.rx(), p.sp())?,
                _ => p.set_sp(p.reg(p.rx())?)?,
            },
            Instruction::INPUT => todo!(),
            Instruction::OUTPUT => todo!(),
            Instruction::OUTCHAR => todo!(),
            Instruction::INCHAR => todo!(),
            Instruction::SOUND => todo!(),
            Instruction::ADD | Instruction::ADDC => {
                p.set_reg(p.rx(), p.reg(p.ry())? + p.reg(p.rz())?)?;

                if *self == Instruction::ADDC {
                    p.set_reg(p.rx(), p.reg(p.rx())? + p.fr(FlagIndex::CARRY)? as usize)?;
                }

                p.set_fr(FlagIndex::ZERO, false)?;
                p.set_fr(FlagIndex::CARRY, false)?;

                match p.reg(p.rx())? {
                    r if r == 0 => p.set_fr(FlagIndex::ZERO, false)?,
                    r if r > MAX_VALUE_MEMORY => {
                        p.set_fr(FlagIndex::CARRY, true)?;
                        p.set_reg(p.rx(), r - MAX_VALUE_MEMORY)?;
                    }
                    _ => (),
                };
            }
            Instruction::SUB | Instruction::SUBC => {
                let sub = if *self == Instruction::SUB {
                    p.reg(p.ry())?.checked_sub(p.reg(p.rz())?)
                } else {
                    p.reg(p.ry())?
                        .checked_sub(p.reg(p.rz())? + p.fr(FlagIndex::CARRY)? as usize)
                };

                match sub {
                    Some(s) => {
                        p.set_reg(p.rx(), s)?;

                        p.set_fr(FlagIndex::ZERO, false)?;
                        p.set_fr(FlagIndex::NEGATIVE, false)?;

                        if s == 0 {
                            p.set_fr(FlagIndex::ZERO, true)?;
                        }
                    }
                    None => {
                        p.set_fr(FlagIndex::ZERO, false)?;
                        p.set_fr(FlagIndex::NEGATIVE, true)?;

                        p.set_reg(p.rx(), 0x0000)?;
                    }
                }
            }
            Instruction::MUL => todo!(),
            Instruction::DIV => todo!(),
            Instruction::INC | Instruction::DEC => {
                if *self == Instruction::INC {
                    p.set_reg(p.rx(), p.reg(p.rx())? + 1)?;
                } else {
                    p.set_reg(p.rx(), p.reg(p.rx())? - 1)?;
                }

                p.set_fr(FlagIndex::ZERO, false)?;

                if p.reg(p.rx())? == 0 {
                    p.set_fr(FlagIndex::ZERO, true)?;
                }
            }
            Instruction::MOD => {
                p.set_reg(p.rx(), p.reg(p.ry())? % p.reg(p.rz())?)?;

                p.set_fr(FlagIndex::ZERO, false)?;

                if p.reg(p.rx())? == 0 {
                    p.set_fr(FlagIndex::ZERO, true)?;
                }
            }
            Instruction::AND | Instruction::OR | Instruction::XOR | Instruction::NOT => {
                match self {
                    Instruction::AND => p.set_reg(p.rx(), p.reg(p.ry())? & p.reg(p.rz)?)?,
                    Instruction::OR => p.set_reg(p.rx(), p.reg(p.ry())? | p.reg(p.rz)?)?,
                    Instruction::XOR => p.set_reg(p.rx(), p.reg(p.ry())? ^ p.reg(p.rz)?)?,
                    Instruction::NOT => p.set_reg(p.rx(), !p.reg(p.ry())?)?,
                    _ => unreachable!(),
                }

                p.set_fr(FlagIndex::ZERO, false)?;

                if p.reg(p.rx())? == 0 {
                    p.set_fr(FlagIndex::ZERO, true)?;
                }
            }
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
            Instruction::NOP => (),
            Instruction::HALT => todo!(),
            Instruction::CLEARC => {
                p.set_fr(FlagIndex::CARRY, false)?;
            }
            Instruction::SETC => {
                p.set_fr(FlagIndex::CARRY, true)?;
            }
            Instruction::BREAKP => todo!(),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_invalid_instruction() {
        let mut p = Processor::debug(10);
        let _ = p.set_mem(0, 0b1011110000000000);
        assert_eq!(
            ProcError::InvalidInstruction(0b1011110000000000),
            p.next().err().unwrap()
        )
    }
}
