pub mod layout {

    use std::ops::RangeInclusive;

    use crate::MemoryCell;

    type Range = RangeInclusive<super::super::MemoryCell>;

    /// Espaço reservado para instruções do programa e variáveis.
    pub const ADDR_PROG_AND_VAR: Range = 0..=16384;

    /// Espaço reservado para os dados estáticos.
    pub const ADDR_STATIC_DATA: Range = 16385..=24576;

    /// Espaço reservado para os dados dinâmicos.
    pub const ADDR_DYNAMIC_DATA: Range = 24577..=30681;

    /// Endereço reservado para chamados do sistema.
    pub const ADDR_SYSTEM_CALL: Range = 30682..=30682;

    /// Espaço reservado para a folga no topo da pilha.
    pub const ADDR_GAP_TOP_STACK: Range = 30683..=30689;

    /// Espaço reservado para a pilha.
    pub const ADDR_STACK: Range = 30690..=32738;

    /// Espaço reservado para a folga na base da pilha.
    pub const ADDR_GAP_BOTTOM_STACK: Range = 32739..=32745;

    pub const ADDR_RX: Range = 32746..=32746;
    pub const ADDR_TX: Range = 32747..=32747;
    pub const ADDR_TIMER: Range = 32748..=32748;
    pub const ADDR_ARGS: Range = 32749..=32758;
    pub const ADDR_RETURN: Range = 32759..=32759;

    pub fn data_area(addr: MemoryCell) -> String {
        match addr {
            a if ADDR_PROG_AND_VAR.contains(&a) => stringify!(ADDR_PROG_AND_VAR).to_string(),
            a if ADDR_STATIC_DATA.contains(&a) => stringify!(ADDR_STATIC_DATA).to_string(),
            a if ADDR_DYNAMIC_DATA.contains(&a) => stringify!(ADDR_DYNAMIC_DATA).to_string(),
            a if ADDR_SYSTEM_CALL.contains(&a) => stringify!(ADDR_SYSTEM_CALL).to_string(),
            a if ADDR_GAP_TOP_STACK.contains(&a) => stringify!(ADDR_GAP_TOP_STACK).to_string(),
            a if ADDR_STACK.contains(&a) => stringify!(ADDR_STACK).to_string(),
            a if ADDR_GAP_BOTTOM_STACK.contains(&a) => {
                stringify!(ADDR_GAP_BOTTOM_STACK).to_string()
            }
            a if ADDR_RX.contains(&a) => stringify!(ADDR_RX).to_string(),
            a if ADDR_TX.contains(&a) => stringify!(ADDR_TX).to_string(),
            a if ADDR_TIMER.contains(&a) => stringify!(ADDR_TIMER).to_string(),
            a if ADDR_ARGS.contains(&a) => stringify!(ADDR_ARGS).to_string(),
            a if ADDR_RETURN.contains(&a) => stringify!(ADDR_RETURN).to_string(),
            _ => "Invalid Data Area".to_string(),
        }
    }
}
