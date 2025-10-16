use std::ops::RangeInclusive;

use isa::{Instruction, InstructionData, MemoryCell};

fn set_bits(mem: MemoryCell, v: MemoryCell, r: RangeInclusive<MemoryCell>) -> MemoryCell {
    let mask = ((1 << (r.end() - r.start() + 1)) - 1) << r.start();
    (mem & !mask) | ((v << r.start()) & mask)
}

fn set_rx(mem: MemoryCell, v: MemoryCell) -> MemoryCell {
    set_bits(mem, v, 7..=9)
}

fn set_ry(mem: MemoryCell, v: MemoryCell) -> MemoryCell {
    set_bits(mem, v, 4..=6)
}

fn set_rz(mem: MemoryCell, v: MemoryCell) -> MemoryCell {
    set_bits(mem, v, 1..=3)
}

pub trait InstructionEncode {
    fn encode(&self) -> Vec<MemoryCell>;
}

impl InstructionEncode for InstructionData {
    fn encode(&self) -> Vec<MemoryCell> {
        match self {
            InstructionData::LOAD { rx, addr1 } => {
                vec![set_rx(Instruction::LOAD.mask_number(), *rx), *addr1]
            }
            InstructionData::LOADN { rx, addr1 } => {
                vec![set_rx(Instruction::LOADN.mask_number(), *rx), *addr1]
            }
            InstructionData::LOADI { rx, ry } => {
                vec![set_ry(set_rx(Instruction::LOADI.mask_number(), *rx), *ry)]
            }
            InstructionData::STORE { rx, addr1 } => {
                vec![set_rx(Instruction::STORE.mask_number(), *rx), *addr1]
            }
            InstructionData::STOREN { rx, addr1 } => {
                vec![set_rx(Instruction::STOREN.mask_number(), *rx), *addr1]
            }
            InstructionData::STOREI { rx, ry } => {
                vec![set_ry(set_rx(Instruction::STOREI.mask_number(), *rx), *ry)]
            }
            InstructionData::MOV { rx, ry, b2_1 } => {
                vec![set_bits(
                    set_ry(set_rx(Instruction::MOV.mask_number(), *rx), *ry),
                    *b2_1,
                    1..=2,
                )]
            }

            InstructionData::INPUT => todo!(),
            InstructionData::OUTPUT => todo!(),
            InstructionData::OUTCHAR => todo!(),
            InstructionData::INCHAR => todo!(),
            InstructionData::SOUND => todo!(),

            InstructionData::ADD { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::ADD.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::ADDC { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::ADDC.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::SUB { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::SUB.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::SUBC { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::SUBC.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::MUL { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::MUL.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::DIV { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::DIV.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::INC { rx } => vec![set_rx(Instruction::INC.mask_number(), *rx)],
            InstructionData::DEC { rx } => vec![set_rx(Instruction::DEC.mask_number(), *rx)],
            InstructionData::MOD { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::MOD.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::AND { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::AND.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::OR { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::OR.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::XOR { rx, ry, rz } => vec![set_rz(
                set_ry(set_rx(Instruction::XOR.mask_number(), *rx), *ry),
                *rz,
            )],
            InstructionData::NOT { rx, ry } => {
                vec![set_ry(set_rx(Instruction::NOT.mask_number(), *rx), *ry)]
            }
            InstructionData::SHIFTL0 { rx, b3_0 } => vec![set_bits(
                set_rx(Instruction::SHIFTL0.mask_number(), *rx),
                *b3_0,
                0..=3,
            )],
            InstructionData::SHIFTL1 { rx, b3_0 } => vec![set_bits(
                set_rx(Instruction::SHIFTL1.mask_number(), *rx),
                *b3_0,
                0..=3,
            )],
            InstructionData::SHIFTR0 { rx, b3_0 } => vec![set_bits(
                set_rx(Instruction::SHIFTR0.mask_number(), *rx),
                *b3_0,
                0..=3,
            )],
            InstructionData::SHIFTR1 { rx, b3_0 } => vec![set_bits(
                set_rx(Instruction::SHIFTR1.mask_number(), *rx),
                *b3_0,
                0..=3,
            )],
            InstructionData::ROTL { rx, ry, b3_0 } => vec![set_bits(
                set_ry(set_rx(Instruction::ROTL.mask_number(), *rx), *ry),
                *b3_0,
                0..=3,
            )],
            InstructionData::ROTR { rx, ry, b3_0 } => vec![set_bits(
                set_ry(set_rx(Instruction::ROTR.mask_number(), *rx), *ry),
                *b3_0,
                0..=3,
            )],
            InstructionData::CMP { rx, ry } => {
                vec![set_ry(set_rx(Instruction::CMP.mask_number(), *rx), *ry)]
            }
            InstructionData::JMP { addr1 } => vec![Instruction::JMP.mask_number(), *addr1],
            InstructionData::JEQ { addr1 } => vec![Instruction::JEQ.mask_number(), *addr1],
            InstructionData::JNE { addr1 } => vec![Instruction::JNE.mask_number(), *addr1],
            InstructionData::JZ { addr1 } => vec![Instruction::JZ.mask_number(), *addr1],
            InstructionData::JNZ { addr1 } => vec![Instruction::JNZ.mask_number(), *addr1],
            InstructionData::JC { addr1 } => vec![Instruction::JC.mask_number(), *addr1],
            InstructionData::JNC { addr1 } => vec![Instruction::JNC.mask_number(), *addr1],
            InstructionData::JGR { addr1 } => vec![Instruction::JGR.mask_number(), *addr1],
            InstructionData::JLE { addr1 } => vec![Instruction::JLE.mask_number(), *addr1],
            InstructionData::JEG { addr1 } => vec![Instruction::JEG.mask_number(), *addr1],
            InstructionData::JEL { addr1 } => vec![Instruction::JEL.mask_number(), *addr1],
            InstructionData::JOV { addr1 } => vec![Instruction::JOV.mask_number(), *addr1],
            InstructionData::JNO { addr1 } => vec![Instruction::JNO.mask_number(), *addr1],
            InstructionData::JDZ { addr1 } => vec![Instruction::JDZ.mask_number(), *addr1],
            InstructionData::JN { addr1 } => vec![Instruction::JN.mask_number(), *addr1],
            InstructionData::CALL { addr1 } => vec![Instruction::CALL.mask_number(), *addr1],
            InstructionData::CEQ { addr1 } => vec![Instruction::CEQ.mask_number(), *addr1],
            InstructionData::CNE { addr1 } => vec![Instruction::CNE.mask_number(), *addr1],
            InstructionData::CZ { addr1 } => vec![Instruction::CZ.mask_number(), *addr1],
            InstructionData::CNZ { addr1 } => vec![Instruction::CNZ.mask_number(), *addr1],
            InstructionData::CC { addr1 } => vec![Instruction::CC.mask_number(), *addr1],
            InstructionData::CNC { addr1 } => vec![Instruction::CNC.mask_number(), *addr1],
            InstructionData::CGR { addr1 } => vec![Instruction::CGR.mask_number(), *addr1],
            InstructionData::CLE { addr1 } => vec![Instruction::CLE.mask_number(), *addr1],
            InstructionData::CEG { addr1 } => vec![Instruction::CEG.mask_number(), *addr1],
            InstructionData::CEL { addr1 } => vec![Instruction::CEL.mask_number(), *addr1],
            InstructionData::COV { addr1 } => vec![Instruction::COV.mask_number(), *addr1],
            InstructionData::CNO { addr1 } => vec![Instruction::CNO.mask_number(), *addr1],
            InstructionData::CDZ { addr1 } => vec![Instruction::CDZ.mask_number(), *addr1],
            InstructionData::CN { addr1 } => vec![Instruction::CN.mask_number(), *addr1],
            InstructionData::RTS => todo!(),
            InstructionData::RTI => todo!(),
            InstructionData::PUSH { rx } => vec![set_rx(Instruction::PUSH.mask_number(), *rx)],
            InstructionData::POP { rx } => vec![set_rx(Instruction::POP.mask_number(), *rx)],
            InstructionData::NOP => todo!(),
            InstructionData::HALT => todo!(),
            InstructionData::CLEARC => todo!(),
            InstructionData::SETC => todo!(),
            InstructionData::BREAKP => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use isa::InstructionData;

    use crate::encoder::InstructionEncode;

    #[test]
    fn test_load_encode() {
        let res = InstructionData::LOAD { rx: 3, addr1: 4 }.encode();
        assert_eq!(0b1100000110000000, res[0]);
        assert_eq!(4, res[1]);
    }
}
