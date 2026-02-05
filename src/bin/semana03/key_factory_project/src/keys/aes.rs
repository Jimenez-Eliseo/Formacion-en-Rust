use crate::Key;
use rand::RngCore;

/// SecurityLevel::Low -> AesKey
pub struct AesKey {
    // Contendra 16 bytes
    bytes: Vec<u8>,
}

impl AesKey {
    // Retorna
    // 16 bytes aleatorios
    pub fn new_128() -> Self {
        let mut bytes = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self { bytes }
    }
}

impl Key for AesKey {
    // devuleve una referencia de slice de los bytes
    // asi que solo podra ver
    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    // Retorna
    // "AES-128"
    fn algorithm_name(&self) -> &str {
        "AES-128"
    }
}

