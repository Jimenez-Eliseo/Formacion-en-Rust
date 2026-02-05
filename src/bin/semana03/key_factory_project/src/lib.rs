pub mod factory;
pub mod keys;

// Nivel de Seguridad
// en un enum para tener mas
// y sera muy escapable
#[derive(Debug, Clone, Copy)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
}

// trait que sera aplicado en lsotres niveles
// de Seguridad que se definio
// en el enum SecurityLevel
// solo seran referencia y mantener
// asi la Seguridad y agilidad
// porque estara en el heap
// y la referencia estara en el stack
// que sera mas rapido el manejo
pub trait Key {
    fn as_bytes(&self) -> &[u8];
    fn algorithm_name(&self) -> &str;
}
