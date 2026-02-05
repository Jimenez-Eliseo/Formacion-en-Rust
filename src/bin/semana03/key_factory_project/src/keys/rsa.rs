use crate::Key;
use rand::RngCore;

pub struct RsaKey {
    // Contendra 512 bytes
    private_key: Vec<u8>,
}

impl RsaKey {
    // Retorna
    // 512 bytes aleatorios
    pub fn new_4096() -> Self {
        let mut bytes = vec![0u8; 512];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self { private_key: bytes }
    }
}

impl Key for RsaKey {
    // devuleve una referencia de slice de los bytes
    // asi que solo podra ver
    fn as_bytes(&self) -> &[u8] {
        &self.private_key
    }

    // Retorna
    // "RSA-409"
    fn algorithm_name(&self) -> &str {
        "RSA-4096"
    }
}

