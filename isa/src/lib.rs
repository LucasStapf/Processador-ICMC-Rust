///! *Instruction Set Architecture - ISA*
use std::ops::RangeInclusive;

const BITS_ADDRESS: usize = 16;

type Opcode = usize;
type MemType = usize;

macro_rules! instruction_set {
    ($($name:ident $op:literal),+) => {

        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum Instruction {
            $($name),+
        }

        impl std::fmt::Display for Instruction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $(Instruction::$name => write!(f, "{}", stringify!($name))),+,
                }
            }
        }

        impl Instruction {
            pub fn opcode(&self) -> Opcode {
                let code = match self {
                    $(Instruction::$name => $op),+,
                };
                Opcode::from_str_radix(&code[0..=5], 2).unwrap()
            }

            pub fn bits(&self, r: RangeInclusive<usize>) -> MemType {
                let code = match self {
                    $(Instruction::$name => $op),+,
                };

                let size = BITS_ADDRESS - 1;

                let cr = RangeInclusive::new(size - r.end(), size - r.start());
                MemType::from_str_radix(&code[cr], 2).unwrap()
            }

            pub fn get_instruction(value: MemType) -> Instruction {
                let value_string = format!("{:016b}", value);

                let mut test_value: String;

                $(test_value = value_string.chars().zip($op.chars()).map(|(v, x)| {
                    if x == 'X' {
                        'X'
                    } else {
                        v
                    }
                }).collect();

                if test_value == $op {
                    return Instruction::$name;
                })+

                return Instruction::InvalidInstruction;
            }

        }
    };
}

instruction_set!(
    InvalidInstruction  "----------------",         // Apenas controle para instrução inválida.
    LOAD        "110000XXXXXXXXXX", // Data Manipulation Instructions
    STORE       "110001XXXXXXXXXX",
    LOADIMED    "111000XXXXXXXXXX",
    STOREIMED   "111001XXXXXXXXXX",
    LOADINDEX   "111100XXXXXXXXXX",
    STOREINDEX  "111101XXXXXXXXXX",
    MOV         "110011XXXXXXXXXX",
    INPUT       "111110XXXXXXXXXX", // Peripheric Instructions
    OUTPUT      "111111XXXXXXXXXX",
    OUTCHAR     "110010XXXXXXXXXX", // IO Instructions
    INCHAR      "110101XXXXXXXXXX",
    SOUND       "110100XXXXXXXXXX",
    ADD         "100000XXXXXXXXXX", // Aritmethic Instructions
    SUB         "100001XXXXXXXXXX",
    MUL         "100010XXXXXXXXXX",
    DIV         "100011XXXXXXXXXX",
    INC         "100100XXXXXXXXXX",
    LMOD        "100101XXXXXXXXXX",
    LAND        "010010XXXXXXXXXX", // Logic Instructions
    LOR         "010011XXXXXXXXXX",
    LXOR        "010100XXXXXXXXXX",
    LNOT        "010101XXXXXXXXXX",
    SHIFT       "010000XXXXXXXXXX",
    CMP         "010110XXXXXXXXXX",
    BRA         "000001XXXXXXXXXX", // Flow Control Instructions
    JMP         "000010XXXXXXXXXX",
    CALL        "000011XXXXXXXXXX",
    RTS         "000100XXXXXXXXX0",
    RTI         "000100XXXXXXXXX1",
    PUSH        "000101XXXXXXXXXX",
    POP         "000110XXXXXXXXXX",
    CALLR       "001001XXXXXXXXXX",
    JMPR        "001010XXXXXXXXXX",
    NOP         "000000XXXXXXXXXX", // Control Instructions
    HALT        "001111XXXXXXXXXX",
    CLEARC      "001000XXXXXXXXXX",
    BREAKP      "001110XXXXXXXXXX");

pub fn bits(mem: MemType, r: RangeInclusive<usize>) -> MemType {
    let mask = (1 << (r.end() - r.start() + 1)) - 1;
    let ret = mem;
    (ret >> r.start()) & mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_display() {
        assert_eq!("LOADINDEX", Instruction::LOADINDEX.to_string());
    }

    #[test]
    fn test_instruction_opcode() {
        assert_eq!(0b110011, Instruction::MOV.opcode());
        assert_eq!(0b000100, Instruction::RTI.opcode());
    }

    #[test]
    fn test_instruction_bits() {
        assert_eq!(0b110011, Instruction::MOV.bits(10..=15));
        assert_eq!(0b1001, Instruction::MOV.bits(11..=14));
    }

    #[test]
    fn test_instruction_get_only_opcode() {
        assert_eq!(
            Instruction::STORE,
            Instruction::get_instruction(0b1100010010010101)
        );
    }

    #[test]
    fn test_instruction_get() {
        assert_eq!(
            Instruction::RTS,
            Instruction::get_instruction(0b0001000000000000)
        );

        assert_eq!(
            Instruction::RTI,
            Instruction::get_instruction(0b0001001111111111)
        );
    }
}
