///! *Instruction Set Architecture - ISA*
use std::ops::RangeInclusive;

pub const BITS_ADDRESS: usize = 16;

/// Tipo de dado utilizado para representar a memória do processador.
pub type MemType = usize;
type Opcode = MemType;

pub struct FlagIndex;

impl FlagIndex {
    pub const GREATER: usize = 0;
    pub const LESSER: usize = 1;
    pub const EQUAL: usize = 2;
    pub const ZERO: usize = 3;
    pub const CARRY: usize = 4;
    pub const ARITHMETIC_OVERFLOW: usize = 5;
    pub const DIV_BY_ZERO: usize = 6;
    pub const STACK_OVERFLOW: usize = 7;
    pub const STACK_UNDERFLOW: usize = 8;
    pub const NEGATIVE: usize = 9;
}

macro_rules! instruction_set {
    ($($(#[$doc:meta])* $name:ident $op:literal),+) => {

        /// Conjunto de instruções presente na Arquitetura do Processador ICMC.
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum Instruction {
            $(
                $(#[$doc])*
                $name
            ),+
        }

        impl std::fmt::Display for Instruction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $(Instruction::$name => write!(f, "{}", stringify!($name))),+,
                }
            }
        }

        impl Instruction {

            pub fn vec() -> Vec<Instruction> {
                let mut vec = Vec::new();
                $(vec.push(Instruction::$name);)+
                vec
            }

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


            pub fn from_str(s: &str) -> Option<Self> {
                $(if s.eq_ignore_ascii_case(stringify!($name)) {
                    return Some(Instruction::$name)
                })+
                None
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
    /// Instrução inválida. Utilizada apenas para sinalização.
    InvalidInstruction  "XXXXXXXXXXXXXXXX",         // Apenas controle para instrução inválida.

    /// Carrega o valor da memória presente no endereço `END` para o registrador `Rx`. 
    ///
    /// # Operação
    /// `Rx` ← MEM(`END`)
    ///
    /// # Uso
    /// ```asm
    /// LOAD Rx, END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// LOAD R3, 0xff00
    /// ```
    LOAD        "110000----------", // Data Manipulation Instruction

    /// Carrega o valor `NR` no registrador `Rx`. 
    ///
    /// # Operação
    /// `Rx` ← `NR`
    ///
    /// # Uso
    /// ```asm
    /// LOADN Rx, #NR
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// LOADN R3, #0xff00
    /// ```
    LOADN       "111000----------",

    /// Carrega o valor da memória presente no endereço armazenado em `Ry` para o registrador
    /// `Rx`. 
    ///
    /// # Operação
    /// `Rx` ← MEM(`Ry`)
    ///
    /// # Uso
    /// ```asm
    /// LOADI Rx, Ry
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// LOADI R3, R0
    /// ```
    LOADI       "111100----------",

    /// Salva no endereço `END` da memória o valor presente no registrador `Rx`.
    ///
    /// # Operação
    /// MEM(`END`) ← Rx
    ///
    /// # Uso
    /// ```asm
    /// STORE END, Rx
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// STORE 0x00ff, R3 
    /// ```
    STORE       "110001----------",

    /// Salva no endereço `END` da memória o valor `NR`.
    ///
    /// # Operação
    /// MEM(`END`) ← `NR`
    ///
    /// # Uso
    /// ```asm
    /// STOREN END, #NR
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// STOREN 0x00ff, #0b10100 
    /// ```
    STOREN      "111001----------",

    /// Salva, na memória, no endereço armazenado em `Rx`, o valor presente no registrador `Ry`.
    ///
    /// # Operação
    /// MEM(`Rx`) ← `Ry`
    ///
    /// # Uso
    /// ```asm
    /// STOREI Rx, Ry
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// STOREI R3, R0 
    /// ```
    STOREI      "111101----------",

    /// Move, para um registrador `Rx` ou para o `SP`, o valor presente em outro registrador.
    ///
    /// # Operação
    /// `Rx` ← `Ry` ou
    /// `Rx` ← `SP` ou
    /// `SP` ← `Rx`
    ///
    /// # Uso
    /// ```asm
    /// MOV Rx, Ry
    /// MOV Rx, SP
    /// MOV SP, Rx
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// MOV R3, R0 
    /// MOV R3, SP 
    /// MOV SP, R0 
    /// ```
    MOV         "110011----------",

    INPUT       "111110----------", // Peripheric Instructions
    OUTPUT      "111111----------",
    OUTCHAR     "110010----------", // IO Instructions
    INCHAR      "110101----------",
    SOUND       "110100----------",

    /// Realiza a soma dos valores presentes nos registradores `Ry` e `Rz`, guardando o resultado
    /// no registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Ry` + `Rz` 
    ///
    /// # Uso
    /// ```asm
    /// ADD Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// ADD R3, R0, R7 
    /// ```
    ADD         "100000---------0", // Aritmethic Instructions

    /// Realiza a soma dos valores presentes nos registradores `Ry` e `Rz` mais o *carry* (`C`),
    /// guardando o resultado no registrador `Rx`. 
    ///
    /// # Operação
    /// `Rx` ← `Ry` + `Rz` + `C`
    ///
    /// # Uso
    /// ```asm
    /// ADDC Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// ADDC R3, R0, R7 
    /// ```
    ADDC        "100000---------1", 

    /// Realiza a subtração dos valores presentes nos registradores `Ry` e `Rz`, guardando o
    /// resultado no registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Ry` - `Rz`
    ///
    /// # Uso
    /// ```asm
    /// SUB Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// SUB R3, R0, R7 
    /// ```
    SUB         "100001---------0",

    /// Realiza a subtração dos valores presentes nos registradores `Ry` e `Rz`, guardando no
    /// registrador `Rx` o resultado somado com o *carry* (`C`).
    ///
    /// # Operação
    /// `Rx` ← `Ry` - `Rz` + `C`
    ///
    /// # Uso
    /// ```asm
    /// SUBC Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// SUBC R3, R0, R7 
    /// ```
    SUBC        "100001---------1",

    /// Realiza a multiplicação dos valores presentes nos registradores `Ry` e `Rz`, guardando o
    /// resultado no registrador `Rx`. 
    ///
    /// # Operação
    /// `Rx` ← `Ry` * `Rz` 
    ///
    /// # Uso
    /// ```asm
    /// MUL Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// MUL R3, R0, R7 
    /// ```
    MUL         "100010---------0",

    /// Realiza a divisão de `Ry` por `Rz`, guardando o resultado no registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Ry` / `Rz` 
    ///
    /// # Uso
    /// ```asm
    /// DIV Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// DIV R3, R0, R7 
    /// ```
    DIV         "100011---------0",

    /// Incrementa em uma unidade o registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Rx` + 1
    ///
    /// # Uso
    /// ```asm
    /// INC Rx
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// INC R3 
    /// ```
    INC         "100100---0------",

    /// Decrementa em uma unidade o registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Rx` - 1
    ///
    /// # Uso
    /// ```asm
    /// DEC Rx
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// DEC R3 
    /// ```
    DEC         "100100---1------",

    /// Realiza a operação de módulo entre os registradores `Ry` e `Rz` e salva o resultado no
    /// registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Ry` % `Rz` 
    ///
    /// # Uso
    /// ```asm
    /// MOD Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// MOD R3, R2, R5 
    /// ```
    MOD         "100101----------",

    /// Realiza a operação *AND* entre os registradores `Ry` e `Rz` e salva o resultado no
    /// registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Ry` & `Rz` 
    ///
    /// # Uso
    /// ```asm
    /// AND Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// AND R3, R2, R5 
    /// ```
    AND         "010010----------", // Logic Instructions
    
    /// Realiza a operação *OR* entre os registradores `Ry` e `Rz` e salva o resultado no
    /// registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Ry` | `Rz` 
    ///
    /// # Uso
    /// ```asm
    /// OR Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// OR R3, R2, R5 
    /// ```
    OR          "010011----------",

    /// Realiza a operação *XOR* entre os registradores `Ry` e `Rz` e salva o resultado no
    /// registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← `Ry` ^ `Rz` 
    ///
    /// # Uso
    /// ```asm
    /// XOR Rx, Ry, Rz
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// XOR R3, R2, R5 
    /// ```
    XOR         "010100----------",

    /// Realiza a operação *NOT* no registrador `Ry` e salva o resultado no registrador `Rx`.
    ///
    /// # Operação
    /// `Rx` ← !`Ry` 
    ///
    /// # Uso
    /// ```asm
    /// NOT Rx, Ry
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// NOT R3, R2 
    /// ```
    NOT         "010101----------",

    SHIFTL0     "010000---000----",
    SHIFTL1     "010000---001----",
    SHIFTR0     "010000---010----",
    SHIFTR1     "010000---011----",
    ROTL        "010000---10-----",
    ROTR        "010000---11-----",

    /// Compara os valores dos registradores `Rx` e `Ry` e atualiza o *flag register* (`FR`) de
    /// acordo com o resultado.
    ///
    /// # Operação
    /// `FR` ← `COND` 
    ///
    /// # Uso
    /// ```asm
    /// CMP Rx, Ry
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CMP R3, R2 
    /// ```
    CMP         "010110----------",

    /// Pula para o endereço `END` da memória.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JMP END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JMP 0x00ff 
    /// ```
    JMP         "0000100000------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::EQUAL`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JEQ END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JEQ 0x00ff 
    /// ```
    JEQ         "0000100001------",
    
    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::EQUAL`] do
    /// *flag register* não estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JNE END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JNE 0x00ff 
    /// ```
    JNE         "0000100010------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::ZERO`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JZ END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JZ 0x00ff 
    /// ```
    JZ          "0000100011------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::ZERO`] do
    /// *flag register* não estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JNZ END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JNZ 0x00ff 
    /// ```
    JNZ         "0000100100------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::CARRY`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JC END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JC 0x00ff 
    /// ```
    JC          "0000100101------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::CARRY`] do
    /// *flag register* não estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JNC END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JNC 0x00ff 
    /// ```
    JNC         "0000100110------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::GREATER`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JGR END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JGR 0x00ff 
    /// ```
    JGR         "0000100111------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::LESSER`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JLE END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JLE 0x00ff 
    /// ```
    JLE         "0000101000------",

    /// Pula para o endereço `END` da memória **se** algum dos *bits* [`FlagIndex::GREATER`] ou
    /// [`FlagIndex::EQUAL`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JEG END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JEG 0x00ff 
    /// ```
    JEG         "0000101001------",

    /// Pula para o endereço `END` da memória **se** algum dos *bits* [`FlagIndex::LESSER`] ou
    /// [`FlagIndex::EQUAL`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JEL END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JEL 0x00ff 
    /// ```
    JEL         "0000101010------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::ARITHMETIC_OVERFLOW`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JOV END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JOV 0x00ff 
    /// ```
    JOV         "0000101011------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::ARITHMETIC_OVERFLOW`] do
    /// *flag register* não estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JNO END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JNO 0x00ff 
    /// ```
    JNO         "0000101100------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::DIV_BY_ZERO`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JDZ END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JDZ 0x00ff 
    /// ```
    JDZ         "0000101101------",

    /// Pula para o endereço `END` da memória **se** o *bit* [`FlagIndex::NEGATIVE`] do
    /// *flag register* estiver setado.
    ///
    /// # Operação
    /// `PC` ← `END` 
    ///
    /// # Uso
    /// ```asm
    /// JN END 
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// JN 0x00ff 
    /// ```
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
