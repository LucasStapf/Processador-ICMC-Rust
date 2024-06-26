///! *Instruction Set Architecture - ISA*
use std::ops::RangeInclusive;
use std::str::FromStr;

use thiserror::Error;

pub mod memory;

pub const BITS_ADDRESS: usize = 16;
pub const MAX_VALUE_MEMORY: usize = 2_usize.pow(BITS_ADDRESS as u32) - 1;

/// Tipo de dado utilizado para representar a memória do processador.
pub type MemoryCell = usize;

type Opcode = MemoryCell;

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

#[derive(Error, Debug, PartialEq)]
#[error("Instrução inválida: {code}")]
pub struct InvalidInstruction {
    code: String,
}

macro_rules! instruction_set {
    ($($(#[$doc:meta])* $name:ident $op:literal),+) => {

        /// Conjunto de instruções presentes na Arquitetura do Processador ICMC.
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum Instruction {
            $(
                $(#[$doc])*
                #[doc = "# Máscara\n"]
                #[doc = "```txt"]
                #[doc = $op]
                #[doc = "```"]
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

        impl FromStr for Instruction {
            type Err = InvalidInstruction;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $(if s.eq_ignore_ascii_case(stringify!($name)) {
                    return Ok(Instruction::$name)
                })+
                Err(Self::Err { code: s.to_string() })
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

            pub fn bits(&self, r: RangeInclusive<usize>) -> MemoryCell {
                let code = match self {
                    $(Instruction::$name => $op),+,
                };

                let size = BITS_ADDRESS - 1;

                let cr = RangeInclusive::new(size - r.end(), size - r.start());
                MemoryCell::from_str_radix(&code[cr], 2).unwrap()
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
            /// assert_eq!(Instruction::LOAD, Instruction::get_instruction(mem).unwrap());
            /// ```
            pub fn get_instruction(value: MemoryCell) -> Result<Instruction, InvalidInstruction> {
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
                    return Ok(Instruction::$name);
                })+

                Err(InvalidInstruction { code: value.to_string() })
            }

        }
    };
}

instruction_set!(
    // /// Instrução inválida. Utilizada apenas para sinalização.
    // InvalidInstruction  "XXXXXXXXXXXXXXXX",         // Apenas controle para instrução inválida.

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
    /// MEM(`END`) ← `Rx`
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

    /// Imprime na tela do processador um *char* mapeado de um arquivo *charmap*. O código do
    /// *pixelmap* que representa o desenho do *char* está codificado no *low-byte* do registrador
    /// `Rx`, enquanto que sua cor se encontra no *high-byte*. A posição do *char* é armazenada
    /// no registrador `Ry`.
    ///
    /// # Cores
    /// As cores mapeadas atualmente com seus respectivos códigos são:
    /// 1. <span style="background-color:white">⠀⠀</span> White --- 0
    /// 2. <span style="background-color:brown">⠀⠀</span> Brown --- 256
    /// 3. <span style="background-color:green">⠀⠀</span> Green --- 512
    /// 4. <span style="background-color:olive">⠀⠀</span> Olive --- 768
    /// 5. <span style="background-color:navy">⠀⠀</span> Navy --- 1024
    /// 6. <span style="background-color:purple">⠀⠀</span> Purple --- 1280
    /// 7. <span style="background-color:teal">⠀⠀</span> Teal --- 1536
    /// 8. <span style="background-color:silver">⠀⠀</span> Silver --- 1792
    /// 9. <span style="background-color:gray">⠀⠀</span> Gray --- 2048
    /// 10. <span style="background-color:red">⠀⠀</span> Red --- 2304
    /// 11. <span style="background-color:lime">⠀⠀</span> Lime --- 2560
    /// 12. <span style="background-color:yellow">⠀⠀</span> Yellow --- 2816
    /// 13. <span style="background-color:blue">⠀⠀</span> Blue --- 3072
    /// 14. <span style="background-color:fuchsia">⠀⠀</span> Fuchsia --- 3328
    /// 15. <span style="background-color:aqua">⠀⠀</span> Aqua --- 3584
    /// 16. <span style="background-color:black">⠀⠀</span> Black --- 3840
    ///
    /// Para imprimir o caracter colorido, basta somar o código do *char* ao código da cor.
    ///
    /// ## Exemplo
    /// * <span style="color:blue">A</span> --- 37 (código da letra A) + 3072 (código da cor azul).
    /// 
    /// # Operação
    /// VÍDEO(`Ry`) ← CHAR(`Rx`)
    ///
    /// # Uso
    /// ```asm
    /// OUTCHAR Rx, Ry
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// OUTCHAR R1, R0
    /// ```
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

    /// Esta operação desliza os bits para a esquerda `N` vezes e os bits que transbordam a
    /// extremidade esquerda desaparecem. Os espaços na direita são preenchidos com 0.
    ///
    /// # Operação
    /// `Rx` ← `Rx` << `N`
    /// ```txt
    /// 1 0 1 0'0 1 1 1   
    ///  ╱ ╱ ╱ ╱ ╱ ╱ ╱
    /// 0 1 0 0'1 1 1 0  
    /// ```
    ///
    /// # Uso
    /// ```asm
    /// SHIFTL0 Rx, N
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// SHIFTL0 R7, 9
    /// ```
    SHIFTL0     "010000---000----",

    /// Esta operação desliza os bits para a esquerda `N` vezes e os bits que transbordam a
    /// extremidade esquerda desaparecem. Os espaços na direita são preenchidos com 1.
    ///
    /// # Operação
    /// `Rx` ← !(!(`Rx`) << `N`)
    /// ```txt
    /// 0 0 1 0'0 1 1 1   
    ///  ╱ ╱ ╱ ╱ ╱ ╱ ╱
    /// 0 1 0 0'1 1 1 1  
    /// ```
    ///
    /// # Uso
    /// ```asm
    /// SHIFTL1 Rx, N
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// SHIFTL1 R7, 9
    /// ```
    SHIFTL1     "010000---001----",

    /// Esta operação desliza os bits para a direita `N` vezes e os bits que transbordam a
    /// extremidade direita desaparecem. Os espaços na esquerda são preenchidos com 0.
    ///
    /// # Operação
    /// `Rx` ← `Rx` >> `N`
    /// ```txt
    /// 0 0 1 0'0 1 1 1  
    ///  ╲ ╲ ╲ ╲ ╲ ╲ ╲    
    /// 0 0 0 1'0 0 1 1  
    /// ```
    ///
    /// # Uso
    /// ```asm
    /// SHIFTR0 Rx, N
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// SHIFTR0 R7, 9
    /// ```
    SHIFTR0     "010000---010----",

    /// Esta operação desliza os bits para a direita `N` vezes e os bits que transbordam a
    /// extremidade direita desaparecem. Os espaços na esquerda são preenchidos com 1.
    ///
    /// # Operação
    /// `Rx` ← !(!(`Rx`) >> `N`)
    /// ```txt
    /// 0 0 1 0'0 1 1 1  
    ///  ╲ ╲ ╲ ╲ ╲ ╲ ╲    
    /// 0 0 0 1'0 0 1 1  
    /// ```
    ///
    /// # Uso
    /// ```asm
    /// SHIFTR1 Rx, N
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// SHIFTR1 R7, 9
    /// ```
    SHIFTR1     "010000---011----",

    /// Esta operação gira os bits para a esquerda `N` vezes e os bits que transbordam para
    /// a extremidade esquerda são reintroduzidos no lado direito.
    ///
    /// # Operação
    /// `Rx` ← (`Rx` << N) | (`Rx` >> ([`BITS_ADDRESS`] - N))
    /// ```txt
    /// ╭─────────────────╮  
    /// │ 0 0 1 0'0 1 1 1 │  
    /// ╰─╯╱ ╱ ╱ ╱ ╱ ╱ ╱╭─╯
    ///   0 1 0 0'1 1 1 0  
    /// ```
    ///
    /// # Uso
    /// ```asm
    /// ROTL Rx, N
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// ROTL R6, 2
    /// ```
    ROTL        "010000---10-----",

    /// Esta operação gira os bits para a direita `N` vezes e os bits que transbordam para
    /// a extremidade direita são reintroduzidos no lado esquerdo.
    ///
    /// # Operação
    /// `Rx` ← (`Rx` >> N) | (`Rx` << ([`BITS_ADDRESS`] - N))
    /// ```txt
    /// ╭─────────────────╮  
    /// │ 0 0 1 0'0 1 1 1 │  
    /// ╰─╮╲ ╲ ╲ ╲ ╲ ╲ ╲╰─╯    
    ///   1 0 0 1'0 0 1 1  
    /// ```
    ///
    /// # Uso
    /// ```asm
    /// ROTL Rx, N
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// ROTL R6, 2
    /// ```
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

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CALL END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CALL 0x003C
    /// ```
    CALL        "0000110000------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::EQUAL`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CEQ END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CEQ 0x003C
    /// ```
    CEQ         "0000110001------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::EQUAL`] do *flag register* não estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CNE END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CNE 0x003C
    /// ```
    CNE         "0000110010------",
    
    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::ZERO`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CZ END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CZ 0x003C
    /// ```
    CZ          "0000110011------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::ZERO`] do *flag register* não estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CNZ END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CNZ 0x003C
    /// ```
    CNZ         "0000110100------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::CARRY`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CC END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CC 0x003C
    /// ```
    CC          "0000110101------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::CARRY`] do *flag register* não estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CNC END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CNC 0x003C
    /// ```
    CNC         "0000110110------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::GREATER`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CGR END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CGR 0x003C
    /// ```
    CGR         "0000110111------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::LESSER`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CLE END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CLE 0x003C
    /// ```
    CLE         "0000111000------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// algum dos *bits* [`FlagIndex::EQUAL`] ou [`FlagIndex::GREATER`] do *flag register* estiver
    /// setado
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CEG END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CEG 0x003C
    /// ```
    CEG         "0000111001------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// algum dos *bits* [`FlagIndex::EQUAL`] ou [`FlagIndex::LESSER`] do *flag register* estiver
    /// setado
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CEL END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CEL 0x003C
    /// ```
    CEL         "0000111010------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::ARITHMETIC_OVERFLOW`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// COV END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// COV 0x003C
    /// ```
    COV         "0000111011------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::ARITHMETIC_OVERFLOW`] do *flag register* não estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CNO END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CNO 0x003C
    /// ```
    CNO         "0000111100------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::DIV_BY_ZERO`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CDZ END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CDZ 0x003C
    /// ```
    CDZ         "0000111101------",

    /// Salva o valor atual do *PC* na *stack* e pula para o endereço do procedimento informado se
    /// o *bit* [`FlagIndex::NEGATIVE`] do *flag register* estiver setado.
    ///
    /// # Operação
    /// MEM(`SP`) ← `PC`  
    /// `PC` ← `END`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// CN END
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CN 0x003C
    /// ```
    CN          "0000111110------",

    /// Altera o valor do *PC* para o último valor salvo na *stack* somado de 1.
    /// 
    /// # Operação
    /// `SP` ← `SP` + 1  
    /// `PC` ← MEM(`SP`)  
    /// `PC` ← `PC` + 1  
    ///
    /// # Uso
    /// ```asm
    /// RTS
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// RTS
    /// ```
    RTS         "000100---------0",

    /// Altera o valor do *PC* para o último valor salvo na *stack*.
    /// 
    /// # Operação
    /// `SP` ← `SP` + 1  
    /// `PC` ← MEM(`SP`)  
    ///
    /// # Uso
    /// ```asm
    /// RTI
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// RTI
    /// ```
    RTI         "000100---------1",

    /// Salva na *stack* o conteúdo de um registrador ou do *flag register*.
    ///
    /// # Operação
    /// MEM(`SP`) ← `Rx`  
    /// `SP` ← `SP` - 1 ou  
    /// MEM(`SP`) ← `FR`  
    /// `SP` ← `SP` - 1  
    ///
    /// # Uso
    /// ```asm
    /// PUSH Rx  
    /// PUSH FR
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// PUSH R5
    /// PUSH FR
    /// ```
    PUSH        "000101----------",

    /// Recupera da *stack* o conteúdo de um registrador ou do *flag register*.
    ///
    /// # Operação
    /// `SP` ← `SP` + 1  
    /// `Rx` ← MEM(`SP`) ou   
    /// `SP` ← `SP` + 1  
    /// `FR` ← MEM(`SP`)    
    ///
    /// # Uso
    /// ```asm
    /// POP Rx  
    /// POP FR
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// POP R5
    /// POP FR
    /// ```
    POP         "000110----------",

    /// Sem operação. Serve apenas para consumir tempo.
    ///
    /// # Operação
    /// Nenhuma
    ///
    /// # Uso
    /// ```asm
    /// NOP
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// NOP
    /// ```
    NOP         "000000----------", // Control Instructions

    /// Para a execução do programa.
    /// 
    /// # Operação
    /// Para o processador
    ///
    /// # Uso
    /// ```asm
    /// HALT
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// HALT
    /// ```
    HALT        "001111----------",

    /// Limpa o bit [`FlagIndex::CARRY`] do *flag register*.
    ///
    /// # Operação
    /// C ← 0
    /// 
    /// # Uso
    /// ```asm
    /// CLEARC
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// CLEARC
    /// ```
    CLEARC      "0010000---------",

    /// Seta o bit [`FlagIndex::CARRY`] do *flag register*.
    ///
    /// # Operação
    /// C ← 1
    /// 
    /// # Uso
    /// ```asm
    /// SETC
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// SETC
    /// ```
    SETC        "0010001---------",

    /// Gera um *breakpoint* no código, forçando o simulador a entrar no modo *debug*.
    ///
    /// # Operação
    /// Nenhuma operação lógica no processador
    ///
    /// # Uso
    /// ```asm
    /// BREAKP
    /// ```
    ///
    /// # Exemplo
    /// ```asm
    /// BREAKP
    /// ```
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
pub fn bits(mem: MemoryCell, r: RangeInclusive<usize>) -> MemoryCell {
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
            Instruction::get_instruction(0b1100010010010101).unwrap()
        );
    }

    #[test]
    fn test_instruction_get() {
        assert_eq!(
            Instruction::RTS,
            Instruction::get_instruction(0b0001000000000000).unwrap()
        );

        assert_eq!(
            Instruction::RTI,
            Instruction::get_instruction(0b0001001111111111).unwrap()
        );
    }
}
