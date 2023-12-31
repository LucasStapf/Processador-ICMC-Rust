use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ProcError {
    MaximumMemoryReached,
    InvalidMemoryIndex(usize),
    InvalidInstruction(usize),
    InvalidRegister(usize),
}

impl Display for ProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcError::MaximumMemoryReached => {
                write!(f, "Limite máximo da memória do processador foi atingido.")
            }
            ProcError::InvalidMemoryIndex(i) => {
                write!(f, "Índice para acesso a memória inválido: {}", i)
            }
            ProcError::InvalidInstruction(m) => {
                write!(f, "Instrução inválida: {:016b}", m)
            }
            ProcError::InvalidRegister(r) => {
                write!(f, "Registrador inválido: {}", r)
            }
        }
    }
}

impl Error for ProcError {}
