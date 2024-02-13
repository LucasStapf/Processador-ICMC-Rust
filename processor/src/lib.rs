#![allow(dead_code, unused_imports, missing_docs)]

pub mod errors;
pub mod instructions;
pub mod modules;

use crate::instructions::InstructionCicle;

use errors::ProcessorError;
use isa::{Instruction, MemoryCell};
use log::{debug, info, warn};
use modules::video::Pixelmap;
use std::{
    borrow::Borrow,
    fmt::Display,
    ops::Deref,
    sync::{Arc, Mutex},
};

/// Tamanho da memória do processador, ou seja, número de endereços disponíveis para o
/// funcionamento do dispositivo.
pub const MEMORY_SIZE: usize = 32768;

/// Número de registradores disponíveis no processador.
pub const NUM_REGISTERS: usize = 8;

pub const MAX_VALUE_MEMORY: usize = 2_usize.pow(isa::BITS_ADDRESS as u32) - 1;

type Result<T> = std::result::Result<T, ProcessorError>;

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
            pc: *isa::memory::layout::ADDR_PROG_AND_VAR.start(),
            ir: 0,
            sp: *isa::memory::layout::ADDR_STACK.end(),
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

    /// Retorna o valor presente no endereço `addr` da memória.
    ///
    /// # Erros
    ///
    /// - [`ProcessorError::InvalidAddress`] caso o endereço seja inválido.
    /// - [`ProcessorError::Generic`] em caso de [`std::sync::PoisonError`].
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
    pub fn mem(&self, addr: MemoryCell) -> Result<MemoryCell> {
        match self.memory.lock() {
            Ok(m) => match m.get(addr) {
                Some(&v) => Ok(v),
                None => Err(ProcessorError::InvalidAddress(addr)),
            },
            Err(e) => Err(ProcessorError::Generic {
                title: "Erro inesperado".to_string(),
                description: e.to_string(),
            }),
        }
    }

    /// Altera o valor salvo no endereço `addr` para `v`.
    ///
    /// # Erros
    ///
    /// - [`ProcessorError::InvalidAddress`] caso o endereço seja inválido.
    /// - [`ProcessorError::Generic`] em caso de [`std::sync::PoisonError`].
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
    pub fn set_mem(&mut self, adrr: MemoryCell, v: MemoryCell) -> Result<()> {
        match self.memory.lock() {
            Ok(mut m) => match m.get_mut(adrr) {
                Some(m) => {
                    *m = v;
                    Ok(())
                }
                None => Err(ProcessorError::InvalidAddress(adrr)),
            },
            Err(e) => Err(ProcessorError::Generic {
                title: "Erro inesperado".to_string(),
                description: e.to_string(),
            }),
        }
    }

    /// Retorna o valor do registrador `n`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcessorError::InvalidRegister`] caso o registrador `n` seja
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
    pub fn reg(&self, n: MemoryCell) -> Result<MemoryCell> {
        match self.registers.get(n) {
            Some(&r) => Ok(r),
            None => Err(ProcessorError::InvalidRegister(n)),
        }
    }

    /// Esta função altera o conteúdo do registrar `n` para `v`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcessorError::InvalidRegister`] caso o registrador `n` seja
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
    pub fn set_reg(&mut self, n: MemoryCell, v: MemoryCell) -> Result<()> {
        match self.registers.get_mut(n) {
            Some(r) => {
                *r = v;
                Ok(())
            }
            None => Err(ProcessorError::InvalidRegister(n)),
        }
    }

    /// Retorna o valor presente no campo `RX` do registrador `IR`.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | RX | RX | RX | -- | -- | -- | -- | -- | -- | -- |
    pub fn rx(&self) -> MemoryCell {
        self.rx
    }

    /// Retorna o valor presente no campo `RY` do registrador `IR`.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | -- | -- | -- | RY | RY | RY | -- | -- | -- | -- |
    pub fn ry(&self) -> MemoryCell {
        self.ry
    }

    /// Retorna o valor presente no campo `RZ` do registrador `IR`.
    ///
    /// # Mapeamento
    ///
    /// | Nº | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
    /// |:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
    /// | IR | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | RZ | RZ | RZ | -- |
    pub fn rz(&self) -> MemoryCell {
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
    /// Retorna o erro [`ProcessorError::InvalidFlag`] caso o valor `i` seja inválido.
    pub fn fr(&self, i: usize) -> Result<bool> {
        match self.fr.get(i) {
            Some(&f) => Ok(f),
            None => Err(ProcessorError::InvalidFlag(i)),
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
    /// Retorna o erro [`ProcessorError::InvalidFlag`] caso o valor `i` seja inválido.
    pub fn set_fr(&mut self, i: usize, v: bool) -> Result<()> {
        match self.fr.get_mut(i) {
            Some(f) => {
                *f = v;
                Ok(())
            }
            None => Err(ProcessorError::InvalidFlag(i)),
        }
    }

    /// Retorna o valor do registrador especial *Instruction Register*.
    pub fn ir(&self) -> MemoryCell {
        self.ir
    }

    /// Retorna o valor do registrador especial *Stack Pointer*.
    pub fn sp(&self) -> MemoryCell {
        self.sp
    }

    /// Altera o valor do registrador especial *Stack Pointer* para `addr`.
    ///
    /// # Erros
    ///
    /// - [`ProcessorError::StackOverflow`] caso o `addr` seja menor que o topo da pilha.
    /// - [`ProcessorError::StackUnderflow`] caso o `addr` seja maior que a base da pilha.
    ///
    pub fn set_sp(&mut self, addr: MemoryCell) -> Result<()> {
        self.sp = addr;
        match addr {
            a if isa::memory::layout::ADDR_STACK.contains(&a) => Ok(()),
            a if a < *isa::memory::layout::ADDR_STACK.start() => {
                Err(ProcessorError::StackOverflow(addr))
            }
            a if a > *isa::memory::layout::ADDR_STACK.end() => {
                Err(ProcessorError::StackUnderflow(addr))
            }
            _ => unreachable!(),
        }
    }

    /// Incrementa o valor do registrador especial *Stack Pointer* de um valor `v`.
    ///
    /// # Erros
    ///
    /// - [`ProcessorError::StackOverflow`] caso o resultado seja menor que o topo da pilha.
    /// - [`ProcessorError::StackUnderflow`] caso o resultado seja maior que a base da pilha.
    pub fn inc_sp(&mut self, v: MemoryCell) -> Result<()> {
        self.set_sp(self.sp() + v)
    }

    /// Decrementa o valor do registrador especial *Stack Pointer* de um valor `v`.
    ///
    /// # Erros
    ///
    /// - [`ProcessorError::StackOverflow`] caso o resultado seja menor que o topo da pilha.
    /// - [`ProcessorError::StackUnderflow`] caso o resultado seja maior que a base da pilha.
    /// - [`ProcessorError::InvalidAddress`] caso o resultado seja negativo.
    pub fn dec_sp(&mut self, v: usize) -> Result<()> {
        match self.sp.checked_sub(v) {
            Some(r) => self.set_sp(r),
            None => Err(ProcessorError::InvalidAddress(0)), // arrumar
        }
    }

    /// Retorna o valor do registrador especial *Program Counter*.
    pub fn pc(&self) -> MemoryCell {
        self.pc
    }

    #[warn(missing_docs)]
    pub fn set_pc(&mut self, addr: usize) -> Result<()> {
        self.pc = addr;
        match addr {
            a if isa::memory::layout::ADDR_PROG_AND_VAR.contains(&a) => Ok(()),
            _ => Err(ProcessorError::SegmentationFault {
                pc: addr,
                data_area: isa::memory::layout::data_area(addr),
            }),
        }
    }

    /// Incrementa o valor do registrador especial *Program Counter* de um valor `v`.
    ///
    /// # Erros
    ///
    /// Retorna [`ProcessorError::SegmentationFault`] caso o resultado esteja fora da área de
    /// programa e variáveis.
    pub fn inc_pc(&mut self, v: usize) -> Result<()> {
        self.set_pc(self.pc + v)
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
