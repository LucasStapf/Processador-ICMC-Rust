use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ProcError {
    ProcessorPanic,
    ChannelClosed,
    ChannelEmpty,
    MaximumMemoryReached,
    InvalidIndex(usize, Option<String>),
    InvalidMemoryIndex(usize),
    InvalidInstruction(usize),
    InvalidRegister(usize),
    Generic(String),
}

impl Display for ProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcError::ProcessorPanic => write!(
                f,
                "Alguma thread emitiu panic enquanto acessava o processador."
            ),
            ProcError::ChannelClosed => write!(f, "Canal fechado"),
            ProcError::ChannelEmpty => write!(f, "Canal vazio"),
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
            ProcError::Generic(s) => write!(f, "{s}"),
        }
    }
}

impl Error for ProcError {}
