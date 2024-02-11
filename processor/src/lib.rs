pub mod errors;
pub mod instructions;
pub mod modules;

use crate::instructions::InstructionCicle;

use isa::{Instruction, MemoryCell};
use log::{debug, info, warn};
use modules::video::Pixelmap;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use self::errors::ProcError;

/// Tamanho da memória do processador, ou seja, número de endereços disponíveis para o
/// funcionamento do dispositivo.
pub const MEMORY_SIZE: usize = 32768;

/// Número de registradores disponíveis no processador.
pub const NUM_REGISTERS: usize = 8;

pub const MAX_VALUE_MEMORY: usize = 2_usize.pow(isa::BITS_ADDRESS as u32) - 1;

type Result<T> = std::result::Result<T, ProcError>;

pub struct Processor {
    memory: Arc<Mutex<Vec<usize>>>, // pub temp
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

    pixel: Option<(Pixelmap, usize)>,

    halted: bool,
    debug: bool,
}

impl Default for Processor {
    fn default() -> Self {
        Processor::new()
    }
}

impl Processor {
    /// Cria um novo processador com [`NUM_REGISTERS`] registradores, memória de tamanho [`MEMORY_SIZE`] e mais 4 registradores especiais que são: *Flag Register* (FR), *Program Counter* (PC), *Stack Pointer* (SP) e *Instruction Register* (IR).
    /// Inicialmente, todos os endereços da memória e todos os registradores guardam o valor 0x0.
    pub fn new() -> Self {
        info!(
            "Novo processador criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            MEMORY_SIZE, NUM_REGISTERS
        );

        let mem = Arc::new(Mutex::new(Vec::with_capacity(MEMORY_SIZE)));
        let c_mem = mem.clone();
        let mut guard = mem.lock().unwrap();
        for _ in 0..MEMORY_SIZE {
            guard.push(0);
        }

        Self {
            memory: c_mem,
            registers: [0; NUM_REGISTERS],
            rx: 0,
            ry: 0,
            rz: 0,
            fr: [false; 16],
            pc: 0,
            ir: 0,
            sp: 0,
            pixel: None,
            halted: false,
            debug: false,
        }
    }

    /// Cria um novo processador para DEBUG com [`NUM_REGISTERS`] registradores, memória de tamanho `s` e mais 4 registradores especiais que são: *Flag Register* (FR), *Program Counter* (PC), *Stack Pointer* (SP) e *Instruction Register* (IR).
    /// Inicialmente, todos os endereços da memória e todos os registradores guardam o valor 0x0.
    ///
    /// # **Importante**
    ///
    /// Deve ser utilizado somente para testes!
    pub fn new_debug(s: usize) -> Self {
        warn!(
            "Novo processador para DEBUG criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            s, NUM_REGISTERS
        );

        let mem = Arc::new(Mutex::new(Vec::with_capacity(MEMORY_SIZE)));
        let c_mem = mem.clone();
        let mut guard = mem.lock().unwrap();
        for _ in 0..s {
            guard.push(0);
        }

        Self {
            memory: c_mem,
            registers: [0; NUM_REGISTERS],
            rx: 0,
            ry: 0,
            rz: 0,
            fr: [false; 16],
            pc: 0,
            ir: 0,
            sp: 0,
            pixel: None,
            halted: false,
            debug: false,
        }
    }

    pub fn arc_mem(&self) -> Arc<Mutex<Vec<MemoryCell>>> {
        self.memory.clone()
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
        match self.memory.lock() {
            Ok(m) => match m.get(i) {
                Some(&v) => Ok(v),
                None => Err(ProcError::InvalidMemoryIndex(i)),
            },
            Err(_) => Err(ProcError::ProcessorPanic),
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
        match self.memory.lock() {
            Ok(mut m) => match m.get_mut(i) {
                Some(m) => {
                    *m = v;
                    Ok(())
                }
                None => Err(ProcError::InvalidMemoryIndex(i)),
            },
            Err(_) => Err(ProcError::ProcessorPanic),
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

    /// Retorna o valor presente no campo `RX` do registrador `IR`.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | RX | RX | RX | -- | -- | -- | -- | -- | -- | -- |
    pub fn rx(&self) -> usize {
        self.rx
    }

    /// Retorna o valor presente no campo `RY` do registrador `IR`.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | -- | -- | -- | RY | RY | RY | -- | -- | -- | -- |
    pub fn ry(&self) -> usize {
        self.ry
    }

    /// Retorna o valor presente no campo `RZ` do registrador `IR`.
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
    /// > ⚠️ **Atenção**  
    /// >
    /// > É recomendado utilizar os índices mapeados em [`isa::FlagIndex`] para evitar erros e
    /// > comportamentos indesejados.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::InvalidIndex`] caso o valor `i` seja inválido.
    pub fn fr(&self, i: usize) -> Result<bool> {
        match self.fr.get(i) {
            Some(&f) => Ok(f),
            None => Err(ProcError::InvalidIndex(
                i,
                Some("Índice do Flag Register inválido".to_string()),
            )),
        }
    }

    /// Altera o valor do *bit* `i` do registrador *Flag Register* para o valor `v`.
    ///
    /// > ⚠️ **Atenção**
    /// >
    /// > É recomendado utilizar os índices mapeados em [`isa::FlagIndex`] para evitar erros e
    /// > comportamentos indesejados.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::InvalidIndex`] caso o valor `i` seja inválido.
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

    /// Incrementa o valor do registrador especial *Stack Pointer* de um valor `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::MaximumMemoryReached`] caso o resultado da
    /// soma seja maior que [`MEMORY_SIZE`] - 1.
    /// É importante notar que neste caso o valor de SP **não** será atualizado.
    pub fn inc_sp(&mut self, v: usize) -> Result<()> {
        return if self.sp + v > MEMORY_SIZE - 1 {
            Err(ProcError::MaximumMemoryReached)
        } else {
            Ok(self.sp += v)
        };
    }

    /// Decrementa o valor do registrador especial *Stack Pointer* de um valor `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::InvalidMemoryIndex`] caso o resultado da
    /// subtração seja menor que 0.
    /// É importante notar que neste caso o valor de SP **não** será atualizado.
    pub fn dec_sp(&mut self, v: usize) -> Result<()> {
        match self.sp.checked_sub(v) {
            Some(r) => Ok(self.sp = r),
            None => Err(ProcError::InvalidMemoryIndex(0)), // arrumar
        }
    }

    /// Retorna o valor do registrador especial *Program Counter*.
    pub fn pc(&self) -> usize {
        self.pc
    }

    #[warn(missing_docs)]
    pub fn set_pc(&mut self, v: usize) -> Result<()> {
        if v > MEMORY_SIZE - 1 {
            Err(ProcError::MaximumMemoryReached)
        } else {
            Ok(self.pc = v)
        }
    }

    /// Incrementa o valor do registrador especial *Program Counter* de um valor `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcError::MaximumMemoryReached`] caso o resultado da
    /// soma seja maior que [`MEMORY_SIZE`].
    /// É importante notar que neste caso o valor de PC **não** será atualizado.
    pub fn inc_pc(&mut self, v: usize) -> Result<()> {
        return if self.pc + v > MEMORY_SIZE - 1 {
            Err(ProcError::MaximumMemoryReached)
        } else {
            self.pc += v;
            Ok(())
        };
    }

    #[warn(missing_docs)]
    pub fn set_halted(&mut self, value: bool) {
        self.halted = value
    }

    #[warn(missing_docs)]
    pub fn halted(&self) -> bool {
        self.halted
    }

    #[warn(missing_docs)]
    pub fn set_debug(&mut self, value: bool) {
        self.debug = value
    }

    #[warn(missing_docs)]
    pub fn debug(&self) -> bool {
        self.halted
    }

    #[warn(missing_docs)]
    pub fn set_pixel(&mut self, value: Option<(Pixelmap, usize)>) {
        self.pixel = value
    }

    #[warn(missing_docs)]
    pub fn pixel(&mut self) -> Option<(Pixelmap, usize)> {
        let p = self.pixel;
        self.pixel = None;
        p
    }

    #[warn(missing_docs)]
    pub fn ula_operation(&mut self) -> Result<()> {
        self.set_fr(isa::FlagIndex::GREATER, false)?;
        self.set_fr(isa::FlagIndex::LESSER, false)?;
        self.set_fr(isa::FlagIndex::EQUAL, false)?;
        self.set_fr(isa::FlagIndex::ZERO, false)?;
        self.set_fr(isa::FlagIndex::CARRY, false)?;
        self.set_fr(isa::FlagIndex::ARITHMETIC_OVERFLOW, false)?;
        self.set_fr(isa::FlagIndex::DIV_BY_ZERO, false)?;
        self.set_fr(isa::FlagIndex::NEGATIVE, false)?;
        Ok(())
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

    #[warn(missing_docs)]
    fn execution_cicle(&mut self) -> Result<()> {
        self.rx = isa::bits(self.ir, 7..=9);
        self.ry = isa::bits(self.ir, 4..=6);
        self.rz = isa::bits(self.ir, 1..=3);

        let instruction = Instruction::get_instruction(self.ir);
        debug!("Execution Cicle [{} {}]", instruction, instruction.mask());
        instruction.execution(self)
    }

    #[warn(missing_docs)]
    pub fn next(&mut self) -> Result<()> {
        if !self.halted {
            self.search_cicle()?;
            let r = self.execution_cicle();
            debug!("{self}");
            r
        } else {
            Ok(())
        }
    }
}

impl Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Registers {:?}] [FR {:?}] [IR {}] [SP {:#06x}] [PC {:#06x}]",
            self.registers, self.fr, self.ir, self.sp, self.pc,
        )
    }
}
