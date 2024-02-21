use crate::{errors::ProcessorError, modules::video};

use super::Processor;
use isa::{FlagIndex, Instruction, MAX_VALUE_MEMORY};

pub trait InstructionCicle {
    fn execution(&self, processor: &mut Processor) -> Result<(), ProcessorError>;
}

impl InstructionCicle for Instruction {
    fn execution(&self, p: &mut Processor) -> Result<(), ProcessorError> {
        match self {
            Instruction::InvalidInstruction => {
                return Err(ProcessorError::InvalidInstruction(p.ir()))
            }

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

            Instruction::OUTCHAR => {
                let index = p.reg(p.ry())?;
                if index > video::VIDEO_BUFFER_LENGHT {
                    return Err(ProcessorError::Generic {
                        title: "Índice inválido na Mémoria de Vídeo".to_string(),
                        description: format!(
                            "O índice {} não pertence ao intervalo de 0 a {}.",
                            index,
                            video::VIDEO_BUFFER_LENGHT
                        ),
                    });
                }

                let c = isa::bits(p.reg(p.rx())?, 0..=7) as u8;
                let color_code = isa::bits(p.reg(p.rx())?, 8..=15);
                let color = video::Color::color(color_code);

                match color {
                    Some(color) => {
                        let pixel = ((c, color), index);
                        p.set_pixel(Some(pixel));
                    }
                    None => {
                        return Err(ProcessorError::Generic {
                            title: "Cor inválida".to_string(),
                            description: format!(
                                "O código {} não representa nenhuma cor mapeada.",
                                color_code
                            ),
                        });
                    }
                }
            }

            Instruction::INCHAR => todo!(),
            Instruction::SOUND => todo!(),

            Instruction::ADD | Instruction::ADDC => {
                p.set_reg(p.rx(), p.reg(p.ry())? + p.reg(p.rz())?)?;

                if *self == Instruction::ADDC {
                    p.set_reg(p.rx(), p.reg(p.rx())? + p.fr(FlagIndex::CARRY)? as usize)?;
                }

                p.ula_operation()?; // Limpa as flags relacionadas as operações
                                    // lógicas-aritméticas

                match p.reg(p.rx())? {
                    r if r > MAX_VALUE_MEMORY => {
                        p.set_fr(FlagIndex::CARRY, true)?;
                        p.set_fr(FlagIndex::ARITHMETIC_OVERFLOW, true)?;
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

                p.ula_operation()?;

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
                let result = if *self == Instruction::INC {
                    p.reg(p.rx())?.checked_add(1)
                } else {
                    p.reg(p.rx())?.checked_sub(1)
                };

                p.ula_operation()?;

                let r = match result {
                    Some(r) => match r {
                        r if r > MAX_VALUE_MEMORY => {
                            p.set_fr(FlagIndex::CARRY, true)?;
                            p.set_fr(FlagIndex::ARITHMETIC_OVERFLOW, true)?;
                            r - MAX_VALUE_MEMORY
                        }
                        r if r == 0 => {
                            p.set_fr(FlagIndex::ZERO, true)?;
                            r
                        }
                        _ => r,
                    },
                    None => {
                        p.set_fr(FlagIndex::NEGATIVE, true)?;
                        0
                    }
                };

                p.set_reg(p.rx(), r)?;
            }

            Instruction::MOD => {
                p.set_reg(p.rx(), p.reg(p.ry())? % p.reg(p.rz())?)?;

                p.ula_operation()?;

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

            Instruction::SHIFTL0
            | Instruction::SHIFTL1
            | Instruction::SHIFTR0
            | Instruction::SHIFTR1 => {
                p.ula_operation()?;

                let mut result = match self {
                    Instruction::SHIFTL0 => p.reg(p.rx())? << isa::bits(p.ir(), 0..=3),
                    Instruction::SHIFTL1 => !(!(p.reg(p.rx())?) << isa::bits(p.ir(), 0..=3)),
                    Instruction::SHIFTR0 => p.reg(p.rx())? >> isa::bits(p.ir(), 0..=3),
                    Instruction::SHIFTR1 => !(!(p.reg(p.rx())?) >> isa::bits(p.ir(), 0..=3)),
                    _ => unreachable!(),
                };

                if result > MAX_VALUE_MEMORY {
                    result = result - MAX_VALUE_MEMORY;
                }

                p.set_reg(p.rx(), result)?;

                if p.reg(p.rx())? == 0 {
                    p.set_fr(FlagIndex::ZERO, true)?;
                }
            }

            Instruction::ROTL | Instruction::ROTR => {
                p.ula_operation()?;
                let result = match self {
                    Instruction::ROTL => {
                        (p.reg(p.rx())? << isa::bits(p.ir(), 0..=3))
                            | ((p.reg(p.rx()))? >> (isa::BITS_ADDRESS - isa::bits(p.ir(), 0..=3)))
                    }
                    Instruction::ROTR => {
                        (p.reg(p.rx())? >> isa::bits(p.ir(), 0..=3))
                            | ((p.reg(p.rx()))? << (isa::BITS_ADDRESS - isa::bits(p.ir(), 0..=3)))
                    }
                    _ => unreachable!(),
                };

                p.set_reg(p.rx(), result)?;
                if p.reg(p.rx())? == 0 {
                    p.set_fr(FlagIndex::ZERO, true)?;
                }
            }

            Instruction::CMP => {
                p.ula_operation()?;
                match p.reg(p.rx())?.cmp(&p.reg(p.ry())?) {
                    std::cmp::Ordering::Less => p.set_fr(FlagIndex::LESSER, true)?,
                    std::cmp::Ordering::Equal => p.set_fr(FlagIndex::EQUAL, true)?,
                    std::cmp::Ordering::Greater => p.set_fr(FlagIndex::GREATER, true)?,
                }
            }

            Instruction::JMP
            | Instruction::JEQ
            | Instruction::JNE
            | Instruction::JZ
            | Instruction::JNZ
            | Instruction::JC
            | Instruction::JNC
            | Instruction::JGR
            | Instruction::JLE
            | Instruction::JEG
            | Instruction::JEL
            | Instruction::JOV
            | Instruction::JNO
            | Instruction::JDZ
            | Instruction::JN => {
                let b = match self {
                    Instruction::JMP => true,
                    Instruction::JEQ if p.fr(FlagIndex::EQUAL)? => true,
                    Instruction::JNE if !p.fr(FlagIndex::EQUAL)? => true,
                    Instruction::JZ if p.fr(FlagIndex::ZERO)? => true,
                    Instruction::JNZ if !p.fr(FlagIndex::ZERO)? => true,
                    Instruction::JC if p.fr(FlagIndex::CARRY)? => true,
                    Instruction::JNC if !p.fr(FlagIndex::CARRY)? => true,
                    Instruction::JGR if p.fr(FlagIndex::GREATER)? => true,
                    Instruction::JLE if p.fr(FlagIndex::LESSER)? => true,
                    Instruction::JEG if p.fr(FlagIndex::EQUAL)? || p.fr(FlagIndex::GREATER)? => {
                        true
                    }
                    Instruction::JEL if p.fr(FlagIndex::EQUAL)? || p.fr(FlagIndex::LESSER)? => true,
                    Instruction::JOV if p.fr(FlagIndex::ARITHMETIC_OVERFLOW)? => true,
                    Instruction::JNO if !p.fr(FlagIndex::ARITHMETIC_OVERFLOW)? => true,
                    Instruction::JDZ if p.fr(FlagIndex::DIV_BY_ZERO)? => true,
                    Instruction::JN if p.fr(FlagIndex::NEGATIVE)? => true,
                    _ => false,
                };

                match b {
                    true => p.set_pc(p.mem(p.pc())?)?,
                    false => p.inc_pc(1)?,
                }
            }

            Instruction::CALL
            | Instruction::CEQ
            | Instruction::CNE
            | Instruction::CZ
            | Instruction::CNZ
            | Instruction::CC
            | Instruction::CNC
            | Instruction::CGR
            | Instruction::CLE
            | Instruction::CEG
            | Instruction::CEL
            | Instruction::COV
            | Instruction::CNO
            | Instruction::CDZ
            | Instruction::CN => {
                let b = match self {
                    Instruction::CALL => true,
                    Instruction::CEQ if p.fr(FlagIndex::EQUAL)? => true,
                    Instruction::CNE if !p.fr(FlagIndex::EQUAL)? => true,
                    Instruction::CZ if p.fr(FlagIndex::ZERO)? => true,
                    Instruction::CNZ if !p.fr(FlagIndex::ZERO)? => true,
                    Instruction::CC if p.fr(FlagIndex::CARRY)? => true,
                    Instruction::CNC if !p.fr(FlagIndex::CARRY)? => true,
                    Instruction::CGR if p.fr(FlagIndex::GREATER)? => true,
                    Instruction::CLE if p.fr(FlagIndex::LESSER)? => true,
                    Instruction::CEG if p.fr(FlagIndex::EQUAL)? || p.fr(FlagIndex::GREATER)? => {
                        true
                    }
                    Instruction::CEL if p.fr(FlagIndex::EQUAL)? || p.fr(FlagIndex::LESSER)? => true,
                    Instruction::COV if p.fr(FlagIndex::ARITHMETIC_OVERFLOW)? => true,
                    Instruction::CNO if !p.fr(FlagIndex::ARITHMETIC_OVERFLOW)? => true,
                    Instruction::CDZ if p.fr(FlagIndex::DIV_BY_ZERO)? => true,
                    Instruction::CN if p.fr(FlagIndex::NEGATIVE)? => true,
                    _ => false,
                };

                match b {
                    true => {
                        p.set_mem(p.sp(), p.pc())?;
                        p.dec_sp(1)?;
                        p.set_pc(p.mem(p.pc())?)?;
                    }
                    false => p.inc_pc(1)?,
                }
            }

            Instruction::RTS => {
                p.inc_sp(1)?;
                p.set_pc(p.mem(p.sp())?)?;
                p.inc_pc(1)?;
            }

            Instruction::RTI => {
                p.inc_sp(1)?;
                p.set_pc(p.mem(p.sp())?)?;
            }

            Instruction::PUSH => {
                match isa::bits(p.ir(), 6..=6) {
                    0 => p.set_mem(p.sp(), p.reg(p.rx())?)?, // Registrador
                    1 => {
                        // FR
                        let mut temp = 0;
                        for i in 0..isa::BITS_ADDRESS {
                            temp += (p.fr(i)? as usize) * 2usize.pow(i as u32)
                        }
                        p.set_mem(p.sp(), temp)?;
                    }
                    _ => unreachable!(),
                };
                p.dec_sp(1)?;
            }

            Instruction::POP => {
                p.inc_sp(1)?;
                match isa::bits(p.ir(), 6..=6) {
                    0 => p.set_reg(p.rx(), p.mem(p.sp())?)?,
                    1 => {
                        for i in 0..isa::BITS_ADDRESS {
                            let b = match isa::bits(p.mem(p.sp())?, i..=i) {
                                0 => false,
                                _ => true,
                            };
                            p.set_fr(i, b)?
                        }
                    }
                    _ => unreachable!(),
                };
            }

            Instruction::NOP => (),
            Instruction::HALT => p.set_status(super::ProcessorStatus::Halted),
            Instruction::CLEARC => {
                p.set_fr(FlagIndex::CARRY, false)?;
            }
            Instruction::SETC => {
                p.set_fr(FlagIndex::CARRY, true)?;
            }
            Instruction::BREAKP => p.set_status(super::ProcessorStatus::Debug),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_invalid_instruction() {
        let mut p = Processor::with_capacity(10);
        let _ = p.set_mem(0, 0b1011110000000000);
        assert_eq!(
            ProcessorError::InvalidInstruction(0b1011110000000000),
            p.instruction_cicle().err().unwrap()
        )
    }
}
