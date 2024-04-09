#![allow(dead_code, unused_imports, missing_docs)]

pub mod errors;
pub mod instructions;

use crate::instructions::InstructionCicle;

use errors::ProcessorError;
use isa::{Instruction, MemoryCell};
use log::{debug, info, warn};

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

/// Tamanho da memória de vídeo do processador.
pub const VRAM_SIZE: usize = 30 * 40 * 8 * 8 * 4;

/// Número de registradores disponíveis no processador.
pub const NUM_REGISTERS: usize = 8;

pub const MAX_VALUE_MEMORY: usize = 2_usize.pow(isa::BITS_ADDRESS as u32) - 1;

type Result<T> = std::result::Result<T, ProcessorError>;

pub struct Processor {
    ram: Vec<usize>,
    vram: Vec<usize>,
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
}

impl Default for Processor {
    fn default() -> Self {
        info!(
            "Novo processador padrão criado. [Tamanho da Memória {}] [Número de Registradores {}]",
            MEMORY_SIZE, NUM_REGISTERS
        );

        let mut mem = Vec::with_capacity(MEMORY_SIZE);
        for _ in 0..MEMORY_SIZE {
            mem.push(0);
        }

        let mut vram = Vec::with_capacity(VRAM_SIZE);
        for _ in 0..vram.capacity() {
            vram.push(0);
        }

        Self {
            ram: mem,
            vram,
            registers: [0; NUM_REGISTERS],
            rx: 0,
            ry: 0,
            rz: 0,
            fr: [false; isa::BITS_ADDRESS],
            pc: *isa::memory::layout::ADDR_PROG_AND_VAR.start(),
            ir: 0,
            sp: *isa::memory::layout::ADDR_STACK.end(),
            status: Arc::new(Mutex::new(ProcessorStatus::Debug)),
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

        let mut mem = Vec::with_capacity(s);
        for _ in 0..s {
            mem.push(0);
        }

        Self {
            ram: mem,
            ..Default::default()
        }
    }

    /// Retorna o valor presente no endereço `addr` da memória.
    ///
    /// # Erros
    ///
    /// - [`ProcessorError::InvalidAddress`] caso o endereço seja inválido.
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
        match self.ram.get(addr) {
            Some(&v) => Ok(v),
            None => Err(ProcessorError::InvalidAddress(addr)),
        }
    }

    /// Altera o valor salvo no endereço `addr` para `v`.
    ///
    /// # Erros
    ///
    /// - [`ProcessorError::InvalidAddress`] caso o endereço seja inválido.
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
        match self.ram.get_mut(adrr) {
            Some(m) => {
                *m = v;
                Ok(())
            }
            None => Err(ProcessorError::InvalidAddress(adrr)),
        }
    }

    pub fn pixel(&self, index: usize) -> Result<(usize, usize, usize, usize)> {
        // Pegar de 4 em 4 valores (RGBA)
        if index % 4 == 0 && index < self.vram.len() {
            Ok((
                self.vram[index],
                self.vram[index + 1],
                self.vram[index + 2],
                self.vram[index + 3],
            ))
        } else {
            Err(ProcessorError::InvalidAddress(index))
        }
    }

    pub fn set_pixel(&mut self, index: usize, rgba: (usize, usize, usize, usize)) -> Result<()> {
        // Pegar de 4 em 4 valores (RGBA)
        if index % 4 == 0 && index < self.vram.len() {
            self.vram[index] = rgba.0;
            self.vram[index + 1] = rgba.1;
            self.vram[index + 2] = rgba.2;
            self.vram[index + 3] = rgba.3;
            Ok(())
        } else {
            Err(ProcessorError::InvalidAddress(index))
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

    /// Realiza a etapa de decodificação do processador.
    ///
    /// # Erros
    ///
    /// Esta função retorna o erro [`ProcessorError::InvalidInstruction`] caso a instrução seja
    /// inválida.
    fn decode_stage(&mut self) -> Result<Instruction> {
        self.rx = isa::bits(self.ir, 7..=9);
        self.ry = isa::bits(self.ir, 4..=6);
        self.rz = isa::bits(self.ir, 1..=3);

        Instruction::get_instruction(self.ir)
            .map_err(|_| ProcessorError::InvalidInstruction(self.ir))
    }

    /// Realiza a etapa de execução do processador.
    ///
    /// # Erros
    ///
    /// Esta função pode retornar [`ProcessorError`].
    fn execution_stage(&mut self, instruction: Instruction) -> Result<()> {
        debug!("Execution Stage [{}]", instruction);
        instruction.execution(self)
    }

    /// Realiza o ciclo de instrução do processador.
    ///
    /// # Erros
    ///
    /// Esta função pode retornar qualquer erro presente em [`ProcessorError`].
    pub fn instruction_cicle(&mut self) -> Result<()> {
        self.fetch_stage()?;
        let inst = self.decode_stage()?;
        self.execution_stage(inst)
    }

    #[warn(missing_docs)]
    pub fn load_memory(&mut self, memory: &[MemoryCell]) -> Result<()> {
        match memory.len() {
            MEMORY_SIZE => {
                self.ram.clear();
                self.ram.copy_from_slice(memory);
                Ok(())
            }
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
        *self
            .status
            .lock()
            .expect("Falha ao trocar o status para o seu valor padrão") = ProcessorStatus::Debug
    }

    #[warn(missing_docs)]
    pub fn reset(&mut self, memory: &[MemoryCell]) -> Result<()> {
        self.reset_fields();
        match memory.len() {
            MEMORY_SIZE => {
                self.ram.clear();
                self.ram.copy_from_slice(memory);
                Ok(())
            }
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
