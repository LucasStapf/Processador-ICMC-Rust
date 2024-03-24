use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RamError {
    #[error("Endereço inválido: {index}")]
    InvalidAddress { index: usize },
}

/// Memória RAM (Random Access Memory) utilizada para guardar as instruções e dados dos programas.
pub struct Ram {
    memory: Vec<usize>,
}

impl Ram {
    /// Cria uma nova RAM com capacidade 0.
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    /// Cria uma nova RAM com capacidade `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory: Vec::with_capacity(capacity),
        }
    }

    /// Carrega novos dados dentro da memória. Qualquer dado anteriormente guardado é apagado.
    pub fn load(&mut self, data: &[usize]) {
        self.memory.clear();
        for d in data {
            self.memory.push(*d);
        }
    }

    /// Retorna o valor presente no endereço `addr`.
    ///
    /// # Erros
    ///
    /// Se o endereço for inválido, o erro [`RamError::InvalidAddress`] é retornado.
    pub fn get(&self, addr: usize) -> Result<usize, RamError> {
        self.memory.get(addr).map_or_else(
            || Err(RamError::InvalidAddress { index: addr }),
            |value| Ok(*value),
        )
    }

    /// Altera o valor presente no endereço `addr` para `value`.
    ///
    /// # Erros
    ///
    /// Se o endereço for inválido, o erro [`RamError::InvalidAddress`] é retornado.
    pub fn set(&mut self, addr: usize, value: usize) -> Result<(), RamError> {
        if let Some(m) = self.memory.get_mut(addr) {
            *m = value;
            Ok(())
        } else {
            Err(RamError::InvalidAddress { index: addr })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ram;

    #[test]
    fn test_get() {
        let memory = [1, 2, 3, 4, 5];
        let mut ram = Ram::new();

        ram.load(&memory);
        assert_eq!(memory[3], ram.get(3).unwrap());
    }

    #[test]
    fn test_set() {
        let memory = [1, 2, 3, 4, 5];
        let mut ram = Ram::new();

        ram.load(&memory);
        ram.set(3, 10).unwrap();
        assert_eq!(10, ram.get(3).unwrap());
    }
}
