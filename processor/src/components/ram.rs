use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RamError {
    #[error("Endereço inválido: {index}")]
    InvalidAddress { index: usize },
    #[error("Tamanho inválido. Esperado um vetor com tamanho {expected}, mas foi recebido um de tamanho {received}.")]
    InvalidLenght { expected: usize, received: usize },
    #[error("Ocorreu uma panic enquanto a memória estava sendo acessada!")]
    Panic,
}

impl<T> From<PoisonError<T>> for RamError {
    fn from(_value: PoisonError<T>) -> Self {
        Self::Panic
    }
}

/// Memória RAM (Random Access Memory) utilizada para guardar as instruções e dados dos programas.
pub struct Ram {
    memory: Arc<Mutex<Vec<usize>>>,
    capacity: usize,
    lenght: usize,
}

impl Ram {
    /// Cria uma nova RAM com capacidade 0.
    pub fn new() -> Self {
        Self {
            memory: Arc::new(Mutex::new(Vec::new())),
            capacity: 0,
            lenght: 0,
        }
    }

    /// Cria uma nova RAM com capacidade `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
            capacity,
            lenght: 0,
        }
    }

    /// Carrega novos dados dentro da memória. Qualquer dado anteriormente guardado é apagado. O
    /// tamanho de `data` deve ser o mesmo que a capacidade da memória RAM.
    ///
    /// # Erros
    ///
    /// Se o tamanho dos dados for diferente da capacidade da memória RAM, esta função irá retornar
    /// o erro [`RamError::InvalidLenght`].
    ///
    /// Se alguma *thread* entrou em pânico enquanto estava acessando a memória, esta função irá
    /// retornar o erro [`RamError::Panic`].
    pub fn load(&mut self, data: &[usize]) -> Result<(), RamError> {
        if data.len() == self.capacity {
            let mut mem = self.memory.lock()?;
            mem.clear();
            for d in data {
                mem.push(*d);
            }
            Ok(())
        } else {
            Err(RamError::InvalidLenght {
                expected: self.capacity,
                received: data.len(),
            })
        }
    }

    /// Retorna o valor presente no endereço `addr`.
    ///
    /// # Erros
    ///
    /// Se o endereço for inválido, o erro [`RamError::InvalidAddress`] é retornado.
    ///
    /// Se alguma *thread* entrou em pânico enquanto estava acessando a memória, esta função irá
    /// retornar o erro [`RamError::Panic`].
    pub fn get(&self, addr: usize) -> Result<usize, RamError> {
        let mem = self.memory.lock()?;
        mem.get(addr).map_or_else(
            || Err(RamError::InvalidAddress { index: addr }),
            |value| Ok(*value),
        )
    }

    /// Altera o valor presente no endereço `addr` para `value`.
    pub fn set(&self, addr: usize, value: usize) -> Result<(), RamError> {
        let mut mem = self.memory.lock()?;
        if let Some(m) = mem.get_mut(addr) {
            *m = value;
            Ok(())
        } else {
            Err(RamError::InvalidAddress { index: addr })
        }
    }
}
