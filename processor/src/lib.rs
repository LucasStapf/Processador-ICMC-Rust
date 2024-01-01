pub mod errors;
pub mod instructions;
use crate::instructions::InstructionCicle;

use isa::Instruction;
use log::{debug, info, warn};
use std::{fmt::Display, usize};

use self::errors::ProcError;

/// Tamanho da memória do processador, ou seja, número de endereços disponíveis para o
/// funcionamento do dispositivo.
pub const MEMORY_SIZE: usize = 32768;

/// Número de registradores disponíveis no processador.
pub const NUM_REGISTERS: usize = 8;

pub const MAX_VALUE_MEMORY: usize = 2_usize.pow(isa::BITS_ADDRESS as u32) - 1;

type Result<T> = std::result::Result<T, ProcError>;

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

pub struct Processor {
    memory: Vec<usize>, // pub temp
    registers: [usize; NUM_REGISTERS],

    rx: usize,
    ry: usize,
    rz: usize,
    // Flag Register
    fr: [bool; 16],
    // Program Counter
    pc: usize,
    // Instruction Register
    ir: usize,
    // Stack Pointer
    sp: usize,
}

impl Processor {
    /// Cria um novo processador com [`NUM_REGISTERS`] registradores, memória de tamanho [`MEMORY_SIZE`] e mais 4 registradores especiais que são: *Flag Register* (FR), *Program Counter* (PC), *Stack Pointer* (SP) e *Instruction Register* (IR).
    /// Inicialmente, todos os endereços da memória e todos os registradores guardam o valor 0x0.
    pub fn new() -> Self {
        info!(
            "Novo processador criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            MEMORY_SIZE, NUM_REGISTERS
        );

        let mut mem = Vec::with_capacity(MEMORY_SIZE);
        for _ in 0..MEMORY_SIZE {
            mem.push(0);
        }

        Self {
            memory: mem,
            registers: [0; NUM_REGISTERS],
            rx: 0,
            ry: 0,
            rz: 0,
            fr: [false; 16],
            pc: 0,
            ir: 0,
            sp: 0,
        }
    }

    /// Cria um novo processador para DEBUG com [`NUM_REGISTERS`] registradores, memória de tamanho `s` e mais 4 registradores especiais que são: *Flag Register* (FR), *Program Counter* (PC), *Stack Pointer* (SP) e *Instruction Register* (IR).
    /// Inicialmente, todos os endereços da memória e todos os registradores guardam o valor 0x0.
    ///
    /// # **Importante**
    ///
    /// Deve ser utilizado somente para testes!
    pub fn debug(s: usize) -> Self {
        warn!(
            "Novo processador para DEBUG criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            s, NUM_REGISTERS
        );

        let mut mem = Vec::with_capacity(s);
        for _ in 0..s {
            mem.push(0);
        }

        Self {
            memory: mem,
            registers: [0; NUM_REGISTERS],
            rx: 0,
            ry: 0,
            rz: 0,
            fr: [false; 16],
            pc: 0,
            ir: 0,
            sp: 0,
        }
    }

    /// Retorna o valor presente na memória de índice `i`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::InvalidMemoryIndex`] caso o índice seja inválido.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use crate::processor::Processor;
    ///
    /// let mut p = Processor::new();
    /// assert_eq!(0x0, p.mem(0).unwrap());
    ///
    /// ```
    pub fn mem(&self, i: usize) -> Result<usize> {
        match self.memory.get(i) {
            Some(&v) => Ok(v),
            None => Err(ProcError::InvalidMemoryIndex(i)),
        }
    }

    /// Altera o valor da memória no índice `i` para `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::InvalidMemoryIndex`] caso o índice `i` seja
    /// inválido.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use crate::processor::Processor;
    ///
    /// let mut p = Processor::new();
    /// assert_eq!(0x0, p.mem(0).unwrap());
    /// p.set_mem(0, 0x1);
    /// assert_eq!(0x1, p.mem(0).unwrap());
    /// ```
    pub fn set_mem(&mut self, i: usize, v: usize) -> Result<()> {
        match self.memory.get_mut(i) {
            Some(m) => {
                *m = v;
                Ok(())
            }
            None => Err(ProcError::InvalidMemoryIndex(i)),
        }
    }

    /// Retorna o valor do registrador `n` (`n` é o índice do vetor de registradores).
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::InvalidRegister`] caso o registrador `n` seja
    /// inválido.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use crate::processor::Processor;
    ///
    /// let mut p = Processor::new();
    /// assert_eq!(0x0, p.reg(0).unwrap());
    /// ```
    pub fn reg(&self, n: usize) -> Result<usize> {
        match self.registers.get(n) {
            Some(&r) => Ok(r),
            None => Err(ProcError::InvalidRegister(n)),
        }
    }

    /// Esta função altera o conteúdo do registrar `n` para `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::InvalidRegister`] caso o registrador `n` seja
    /// invalido.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use crate::processor::Processor;
    ///
    /// let mut p = Processor::new();
    /// assert_eq!(0x0, p.reg(0).unwrap());
    /// p.set_reg(0, 0x1);
    /// assert_eq!(0x1, p.reg(0).unwrap());
    /// ```
    pub fn set_reg(&mut self, n: usize, v: usize) -> Result<()> {
        match self.registers.get_mut(n) {
            Some(r) => {
                *r = v;
                Ok(())
            }
            None => Err(ProcError::InvalidRegister(n)),
        }
    }

    /// Retorna o valor presente no campo RX do registrador IR.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | RX | RX | RX | -- | -- | -- | -- | -- | -- | -- |
    pub fn rx(&self) -> usize {
        self.rx
    }

    /// Retorna o valor presente no campo RY do registrador IR.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | -- | -- | -- | RY | RY | RY | -- | -- | -- | -- |
    pub fn ry(&self) -> usize {
        self.ry
    }

    /// Retorna o valor presente no campo RZ do registrador IR.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | RZ | RZ | RZ | -- |
    pub fn rz(&self) -> usize {
        self.rz
    }

    /// Retorna o valor da `i`-ésima *flag* do registrador *Flag Register*.
    ///
    /// > ⚠️  **Atenção** :É recomendo utilizar os índices mapeados em [`FlagIndex`] para evitar
    /// erros e comportamentos indesejados.
    pub fn fr(&self, i: usize) -> Result<bool> {
        match self.fr.get(i) {
            Some(&f) => Ok(f),
            None => Err(ProcError::InvalidIndex(
                i,
                Some("Índice do Flag Register inválido".to_string()),
            )),
        }
    }

    pub fn set_fr(&mut self, i: usize, v: bool) -> Result<()> {
        match self.fr.get_mut(i) {
            Some(f) => {
                *f = v;
                Ok(())
            }
            None => Err(ProcError::InvalidIndex(
                i,
                Some("Índice do Flag Register inválido".to_string()),
            )),
        }
    }

    /// Retorna o valor do registrador especial *Instruction Register*.
    pub fn ir(&self) -> usize {
        self.ir
    }

    /// Retorna o valor do registrador especial *Stack Pointer*.
    pub fn sp(&self) -> usize {
        self.sp
    }

    /// Altera o valor do registrador especial *Stack Pointer* para `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::MaximumMemoryReached`] caso o valor `v` seja
    /// maior que [`MEMORY_SIZE`].
    pub fn set_sp(&mut self, v: usize) -> Result<()> {
        return if v > MEMORY_SIZE {
            Err(ProcError::MaximumMemoryReached)
        } else {
            self.sp = v;
            Ok(())
        };
    }

    /// Retorna o valor do registrador especial *Program Counter*.
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Incrementa o valor do registrador especial *Program Counter* de um valor `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::MaximumMemoryReached`] caso o resultado da
    /// soma seja maior que [`MEMORY_SIZE`].
    /// É importante notar que neste caso o valor de PC **não** será atualizado.
    pub fn inc_pc(&mut self, v: usize) -> Result<()> {
        return if self.pc + v > MEMORY_SIZE {
            Err(ProcError::MaximumMemoryReached)
        } else {
            self.pc += v;
            Ok(())
        };
    }

    /// Realiza o ciclo de busca do processador.
    ///
    /// # Erros
    ///
    /// Esta função pode retornar qualquer um dos erros abaixo:
    ///
    /// - [`ProcError::InvalidMemoryIndex`] - Caso o ciclo de busca ocorra sobre um índice
    /// inválido.
    /// - [`ProcError::MaximumMemoryReached`] - Caso o limite de memória seja atingido.
    fn search_cicle(&mut self) -> Result<()> {
        self.ir = self.mem(self.pc)?;

        debug!(
            "Search Cicle [Instruction Register {:016b}] [Program Counter {}]",
            self.ir, self.pc
        );

        self.inc_pc(1)?;

        Ok(())
    }

    fn execution_cicle(&mut self) -> Result<()> {
        self.rx = isa::bits(self.ir, 7..=9);
        self.ry = isa::bits(self.ir, 4..=6);
        self.rz = isa::bits(self.ir, 1..=3);

        let instruction = Instruction::get_instruction(self.ir);
        debug!("Execution Cicle [{} {}]", instruction, instruction.mask());
        instruction.execution(self)?;
        Ok(())
    }

    pub fn next(&mut self) -> Result<()> {
        self.search_cicle()?;
        self.execution_cicle()?;
        debug!("{self}");
        Ok(())
    }

    pub fn state(
        &self,
    ) -> (
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) {
        (
            self.registers[0],
            self.registers[1],
            self.registers[2],
            self.registers[3],
            self.registers[4],
            self.registers[5],
            self.registers[6],
            self.registers[7],
            self.pc,
            self.sp,
            self.ir,
        )
    }
}

impl Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Registers {:?}] [FR {:?}] [IR {}] [SP {}] [PC {}]",
            self.registers, self.fr, self.ir, self.sp, self.pc,
        )
    }
}
