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

            /// Retorna o OPCODE da instrução.
            pub fn opcode(&self) -> Opcode {
                let code = match self {
                    $(Instruction::$name => $op),+,
                };
                Opcode::from_str_radix(&code[0..=5], 2).unwrap()
            }

            /// Retorna a máscara da instrução.
            pub fn mask(&self) -> &str {
                match self {
                    $(Instruction::$name => $op),+,
                }
            }

            pub fn bits(&self, r: RangeInclusive<usize>) -> MemType {
                let code = match self {
                    $(Instruction::$name => $op),+,
                };

                let size = BITS_ADDRESS - 1;

                let cr = RangeInclusive::new(size - r.end(), size - r.start());
                MemType::from_str_radix(&code[cr], 2).unwrap()
            }

            /// Retorna qual [`Instruction`] está presente no argumento `value`.
            /// Se a instrução for inválida, irá retornar [`Instruction::InvalidInstruction`].
            ///
            /// ## Exemplo
            ///
            /// ```
            /// use isa::*;
            ///
            /// let mem = 0b1100001000100011; // LOAD
            /// assert_eq!(Instruction::LOAD, Instruction::get_instruction(mem));
            /// ```
            pub fn get_instruction(value: MemType) -> Instruction {
                let value_string = format!("{:016b}", value);

                let mut test_value: String;

                $(test_value = value_string.chars().zip($op.chars()).map(|(v, x)| {
                    if x == '-' {
                        '-'
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
    InvalidInstruction  "XXXXXXXXXXXXXXXX",         // Apenas controle para instrução inválida.
    LOAD        "110000----------", // Data Manipulation Instructions
    LOADN       "111000----------",
    LOADI       "111100----------",
    STORE       "110001----------",
    STOREN      "111001----------",
    STOREI      "111101----------",
    MOV         "110011----------",
    INPUT       "111110----------", // Peripheric Instructions
    OUTPUT      "111111----------",
    OUTCHAR     "110010----------", // IO Instructions
    INCHAR      "110101----------",
    SOUND       "110100----------",
    ADD         "100000---------0", // Aritmethic Instructions
    ADDC        "100000---------1", 
    SUB         "100001---------0",
    SUBC        "100001---------1",
    MUL         "100010---------0",
    DIV         "100011---------0",
    INC         "100100---0------",
    DEC         "100100---1------",
    MOD         "100101----------",
    AND         "010010----------", // Logic Instructions
    OR          "010011----------",
    XOR         "010100----------",
    NOT         "010101----------",
    SHIFTL0     "010000---000----",
    SHIFTL1     "010000---001----",
    SHIFTR0     "010000---010----",
    SHIFTR1     "010000---011----",
    ROTL        "010000---10-----",
    ROTR        "010000---11-----",
    CMP         "010110----------",
    JMP         "0000100000------",
    JEQ         "0000100001------",
    JNE         "0000100010------",
    JZ          "0000100011------",
    JNZ         "0000100100------",
    JC          "0000100101------",
    JNC         "0000100110------",
    JGR         "0000100111------",
    JLE         "0000101000------",
    JEG         "0000101001------",
    JEL         "0000101010------",
    JOV         "0000101011------",
    JNO         "0000101100------",
    JDZ         "0000101101------",
    JN          "0000101110------",
    CALL        "0000110000------",
    CEQ         "0000110001------",
    CNE         "0000110010------",
    CZ          "0000110011------",
    CNZ         "0000110100------",
    CC          "0000110101------",
    CNC         "0000110110------",
    CGR         "0000110111------",
    CLE         "0000111000------",
    CEG         "0000111001------",
    CEL         "0000111010------",
    COV         "0000111011------",
    CNO         "0000111100------",
    CDZ         "0000111101------",
    CN          "0000111110------",
    RTS         "000100---------0",
    RTI         "000100---------1",
    PUSH        "000101----------",
    POP         "000110----------",
    NOP         "000000----------", // Control Instructions
    HALT        "001111----------",
    CLEARC      "0010000---------",
    SETC        "0010001---------",
    BREAKP      "001110----------");

/// Retorna os bits presentes no valor `mem` que estão no range `r`.
/// A contagem começa do *low bit* para o *high bit*.
///
/// ## Exemplo:
///
/// ```
/// use isa::*;
///
/// //  bits:   543210
/// let mem = 0b101000;
/// assert_eq!(0b101, bits(mem, 3..=5));
/// ```
pub fn bits(mem: MemType, r: RangeInclusive<usize>) -> MemType {
    let mask = (1 << (r.end() - r.start() + 1)) - 1;
    let ret = mem;
    (ret >> r.start()) & mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        assert_eq!(0b11, bits(0b110000, 4..=5));
    }

    #[test]
    fn test_instruction_display() {
        assert_eq!("LOADI", Instruction::LOADI.to_string());
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
