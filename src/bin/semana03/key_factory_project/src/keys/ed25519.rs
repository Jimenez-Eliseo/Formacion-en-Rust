use crate::Key;
use rand::RngCore;

/// SecurityLevel::Medium -> Ed25519Key
pub struct Ed25519Key {
    // Contendra 32 bytes
    private_key: Vec<u8>,
}

impl Ed25519Key {
    pub fn new() -> Self {
        // Retorna
        // 32 bytes aleatorios
        let mut bytes = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self { private_key: bytes }
    }
}

impl Key for Ed25519Key {
    // devuleve una referencia de slice de los bytes
    // asi que solo podra ver
    fn as_bytes(&self) -> &[u8] {
        &self.private_key
    }

    // Retorna
    // "Ed25519"
    fn algorithm_name(&self) -> &str {
        "Ed25519"
    }
}
