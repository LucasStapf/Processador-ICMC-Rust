#![allow(dead_code, unused_imports, missing_docs)]

pub mod errors;
pub mod instructions;
pub mod modules;

use crate::instructions::InstructionCicle;

use errors::ProcessorError;
use isa::{Instruction, MemoryCell};
use log::{debug, info, warn};
use modules::{
    control_unit::ControlUnit,
    video::{Pixelmap, VideoModule},
};
use std::{
    borrow::Borrow,
    fmt::Display,
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone, Copy)]
pub enum ProcessorStatus {
    Running,
    Debug,
    Halted,
}

/// Tamanho da memória do processador, ou seja, número de endereços disponíveis para o
/// funcionamento do dispositivo.
pub const MEMORY_SIZE: usize = 32768;

/// Número de registradores disponíveis no processador.
pub const NUM_REGISTERS: usize = 8;

pub const MAX_VALUE_MEMORY: usize = 2_usize.pow(isa::BITS_ADDRESS as u32) - 1;

type Result<T> = std::result::Result<T, ProcessorError>;

pub struct Processor {
    memory: Arc<Mutex<Vec<usize>>>, // pub temp
    cu: ControlUnit,
    video: Arc<Mutex<VideoModule>>,

    registers: [usize; NUM_REGISTERS],

    rx: usize,
    ry: usize,
    rz: usize,
    // Flag Register
    fr: [bool; isa::BITS_ADDRESS],
    // Program Counter
    pc: usize,
    // Instruction Register
    ir: usize,
    // Stack Pointer
    sp: usize,

    status: Arc<Mutex<ProcessorStatus>>,
    pixel: Option<(Pixelmap, usize)>,
}

impl Default for Processor {
    fn default() -> Self {
        info!(
            "Novo processador padrão criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            MEMORY_SIZE, NUM_REGISTERS
        );

        let mem = Arc::new(Mutex::new(Vec::with_capacity(MEMORY_SIZE)));
        let mut guard = mem.lock().unwrap();
        for _ in 0..MEMORY_SIZE {
            guard.push(0);
        }
        drop(guard);

        Self {
            memory: mem,
            cu: ControlUnit::default(),
            video: Arc::new(Mutex::new(VideoModule::default())),
            registers: [0; NUM_REGISTERS],
            rx: 0,
            ry: 0,
            rz: 0,
            fr: [false; isa::BITS_ADDRESS],
            pc: *isa::memory::layout::ADDR_PROG_AND_VAR.start(),
            ir: 0,
            sp: *isa::memory::layout::ADDR_STACK.end(),
            status: Arc::new(Mutex::new(ProcessorStatus::Debug)),
            pixel: None,
        }
    }
}

impl Processor {
    /// Cria um novo processado com [`NUM_REGISTERS`] registradores, memória de tamanho
    /// [`MEMORY_SIZE`], iniciada com zeros, e mais 4 registradores especiais:
    ///
    /// * *Program Counter* (PC): 0
    /// * *Stack Pointer* (SP): 32738
    /// * *Instruction Register* (IR): 0
    /// * *Flag Register* (FR): 0
    ///
    pub fn new() -> Self {
        info!(
            "Novo processador criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            MEMORY_SIZE, NUM_REGISTERS
        );

        Self::default()
    }

    /// Cria um novo processado com [`NUM_REGISTERS`] registradores, memória de tamanho
    /// `s`, iniciada com zeros, e mais 4 registradores especiais:
    ///
    /// * *Program Counter* (PC): 0
    /// * *Stack Pointer* (SP): 32738
    /// * *Instruction Register* (IR): 0
    /// * *Flag Register* (FR): 0
    ///
    /// # **Importante**
    ///
    /// Deve ser utilizado somente para testes!
    pub fn with_capacity(s: usize) -> Self {
        warn!(
            "Novo processador para DEBUG criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            s, NUM_REGISTERS
        );

        let mem = Arc::new(Mutex::new(Vec::with_capacity(s)));
        let mut guard = mem.lock().unwrap();
        for _ in 0..s {
            guard.push(0);
        }
        drop(guard);

        Self {
            memory: mem,
            ..Default::default()
        }
    }

    pub fn run(processor: Arc<Mutex<Processor>>) {
        thread::spawn(move || {
            while let Ok(mut p) = processor.lock() {
                match p.status() {
                    ProcessorStatus::Running => match p.instruction_cicle() {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    },
                    ProcessorStatus::Halted | ProcessorStatus::Debug => break,
                }
            }
        });
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
    /// let mut p = Processor::with_capacity(10);
    /// assert_eq!(0x0, p.mem(0).unwrap());
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
    /// let mut p = Processor::with_capacity(10);
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
    /// let mut p = Processor::with_capacity(10);
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
    /// let mut p = Processor::with_capacity(10);
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

    /// Altera o valor do registrador especial *Program Counter* para `addr`.
    ///
    /// # Erros
    ///
    /// Retorna o erro [`ProcessorError::SegmentationFault`] caso o **PC** aponte para uma área
    /// proibida para ele.
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

    pub fn set_status(&mut self, status: ProcessorStatus) {
        *self.status.lock().expect("Falha ao trocar o status atual") = status
    }

    pub fn status(&self) -> ProcessorStatus {
        *self.status.lock().expect("Falha ao acessar o status atual")
    }

    #[warn(missing_docs)]
    pub fn set_pixel(&mut self, value: Option<(Pixelmap, usize)>) {
        self.pixel = value
    }

    /// Retorna o último *pixel* modificado pelo processador. Após a leitura, o *pixel* é descartado.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use crate::processor::Processor;
    /// use crate::processor::modules::video::Color;   
    ///
    /// let mut p = Processor::with_capacity(10);
    /// p.set_pixel(Some(((2, Color::Black), 0)));
    /// assert!(p.pixel().is_some());
    /// assert!(p.pixel().is_none());
    /// ```
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

    /// Realiza a etapa de busca do processador.
    ///
    /// # Erros
    ///
    /// Esta função pode retornar qualquer um dos erros abaixo:
    ///
    /// - [`ProcError::InvalidMemoryIndex`] - Caso a etapa de busca ocorra sobre um índice
    /// inválido.
    /// - [`ProcError::MaximumMemoryReached`] - Caso o limite de memória seja atingido.
    fn fetch_stage(&mut self) -> Result<()> {
        self.ir = self.mem(self.pc)?;
        debug!(
            "Fetch Stage [Instruction Register {:016b}] [Program Counter {}]",
            self.ir, self.pc
        );
        self.inc_pc(1)?;
        Ok(())
    }

    #[warn(missing_docs)]
    fn decode_stage(&mut self) -> Result<Instruction> {
        self.rx = isa::bits(self.ir, 7..=9);
        self.ry = isa::bits(self.ir, 4..=6);
        self.rz = isa::bits(self.ir, 1..=3);

        let instruction = Instruction::get_instruction(self.ir);
        debug!("Decode Stage [{} {}]", instruction, instruction.mask());
        match instruction {
            Instruction::InvalidInstruction => Err(ProcessorError::InvalidInstruction(self.ir)),
            _ => Ok(instruction),
        }
    }

    #[warn(missing_docs)]
    fn execution_stage(&mut self, instruction: Instruction) -> Result<()> {
        debug!("Execution Stage [{}]", instruction);
        instruction.execution(self)
    }

    #[warn(missing_docs)]
    pub fn instruction_cicle(&mut self) -> Result<()> {
        self.fetch_stage()?;
        let inst = self.decode_stage()?;
        self.execution_stage(inst)
    }

    // #[warn(missing_docs)]
    // fn execution_cicle(&mut self) -> Result<()> {
    //     self.rx = isa::bits(self.ir, 7..=9);
    //     self.ry = isa::bits(self.ir, 4..=6);
    //     self.rz = isa::bits(self.ir, 1..=3);
    //
    //     let instruction = Instruction::get_instruction(self.ir);
    //     debug!("Execution Cicle [{} {}]", instruction, instruction.mask());
    //     instruction.execution(self)
    // }

    // #[warn(missing_docs)]
    // pub fn next(&mut self) -> Result<()> {
    //     if !self.halted {
    //         self.instruction_cicle()?;
    //     }
    //
    //     Ok(())
    // }

    #[warn(missing_docs)]
    pub fn load_memory(&mut self, memory: &[MemoryCell]) -> Result<()> {
        match memory.len() {
            MEMORY_SIZE => match self.memory.lock() {
                Ok(mut m) => {
                    m.clear();
                    m.copy_from_slice(memory);
                    Ok(())
                }
                Err(e) => Err(ProcessorError::Generic {
                    title: "Poison error".to_string(),
                    description:
                        format!("Uma thread entrou em pânico enquanto acessava a memória do processador: {e}"),
                }),
            },
            _ => Err(ProcessorError::Generic {
                title: "Tamanho inválido de memória".to_string(),
                description:
                    "Uma memória de tamanho não permitido tentou ser carregada no processador."
                        .to_string(),
            }),
        }
    }

    #[warn(missing_docs)]
    fn reset_fields(&mut self) {
        self.pc = *isa::memory::layout::ADDR_PROG_AND_VAR.start();
        self.ir = 0;
        self.sp = *isa::memory::layout::ADDR_STACK.end();
        self.fr = [false; isa::BITS_ADDRESS];
        self.rx = 0;
        self.ry = 0;
        self.rz = 0;
        self.pixel = None;
        *self
            .status
            .lock()
            .expect("Falha ao trocar o status para o seu valor padrão") = ProcessorStatus::Debug
    }

    #[warn(missing_docs)]
    pub fn reset(&mut self, memory: &[MemoryCell]) -> Result<()> {
        self.reset_fields();
        match memory.len() {
            MEMORY_SIZE => match self.memory.lock() {
                Ok(mut m) => {
                    m.clear();
                    m.copy_from_slice(memory);
                    Ok(())
                }
                Err(e) => Err(ProcessorError::Generic {
                    title: "Poison error".to_string(),
                    description:
                        format!("Uma thread entrou em pânico enquanto acessava a memória do processador: {e}"),
                }),
            },
            _ => Err(ProcessorError::Generic {
                title: "Tamanho inválido de memória".to_string(),
                description:
                    "Uma memória de tamanho não permitido tentou ser carregada no processador."
                        .to_string(),
            }),
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
