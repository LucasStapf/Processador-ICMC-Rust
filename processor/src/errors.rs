use std::{
    error::{self, Error},
    fmt::Display,
};

use isa::MemoryCell;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ProcessorError {
    #[error("O registrador PC({pc}) tentou acessar uma área indevida: {data_area}")]
    SegmentationFault { pc: MemoryCell, data_area: String },

    #[error(
        "O registrador SP({0}) ultrapassou seu limite. Área da pilha: {:?}",
        isa::memory::layout::ADDR_STACK
    )]
    StackOverflow(MemoryCell),

    #[error(
        "O registrador SP({0}) ultrapassou seu limite. Área da pilha: {:?}",
        isa::memory::layout::ADDR_STACK
    )]
    StackUnderflow(MemoryCell),

    #[error("Endereço inválido: {0}")]
    InvalidAddress(MemoryCell),

    #[error("Instrução inválida: {0}")]
    InvalidInstruction(MemoryCell),

    #[error("Registrador inválido: {0}")]
    InvalidRegister(MemoryCell),

    #[error("Flag inválida: {0}")]
    InvalidFlag(usize),

    #[error("{title}: {description}")]
    Generic { title: String, description: String },
}
