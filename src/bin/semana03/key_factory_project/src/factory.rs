use crate::keys::{aes::AesKey, ed25519::Ed25519Key, rsa::RsaKey};
use crate::{Key, SecurityLevel};

pub struct KeyFactory;

impl KeyFactory {
    // retorna a un obsjeto que implemente el
    // trait Key que al estar en el Box
    // resivira un puntero que este vivira en el stack
    // y lo que anda referenciando estara en el heap
    // este sera evalucaod en el match por el nivel que
    // Low -> Genera AesKey (128 bits)
    // Medium -> Genera Ed25519Key
    // High -> Genera RsaKey (4096 bits)
    pub fn generate_key(level: SecurityLevel) -> Box<dyn Key> {
        match level {
            SecurityLevel::Low => Box::new(AesKey::new_128()),
            SecurityLevel::Medium => Box::new(Ed25519Key::new()),
            SecurityLevel::High => Box::new(RsaKey::new_4096()),
        }
    }
}
