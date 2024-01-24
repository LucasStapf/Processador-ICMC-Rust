use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ProcError {
    BlockedMemory,
    MaximumMemoryReached,
    InvalidIndex(usize, Option<String>),
    InvalidMemoryIndex(usize),
    InvalidInstruction(usize),
    InvalidRegister(usize),
}

impl Display for ProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcError::BlockedMemory => write!(f, "Memória bloqueada!"),
            ProcError::MaximumMemoryReached => {
                write!(f, "Limite máximo da memória do processador foi atingido.")
            }
            ProcError::InvalidIndex(i, s) => match s {
                Some(s) => write!(f, "[índice: {}] {}", i, s),
                None => write!(f, "Índice inválido: {}", i),
            },
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
