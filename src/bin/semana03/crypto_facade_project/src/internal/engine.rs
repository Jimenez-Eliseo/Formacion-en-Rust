use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use rand::RngCore;

use crate::internal::error_handler::CryptoError;

pub struct CipherEngine;

impl CipherEngine {
    // genera un IV (nonce) aleatorio
    // AES-GCM exige nonces unicos
    pub fn generate_iv() -> [u8; 12] {
        let mut iv = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut iv);
        iv
    }

    // cifra datos usando AES 256 GCM
    // lo vuleve nebinario y sera manejado en otra capa
    pub fn encrypt(key: &[u8; 32], iv: &[u8; 12], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // creamos el objeto AES
        // devuelve Result y tratamos esto se podria hacer
        // de manera mas simplificada pero para poder
        // entender lecodigo uso el match
        let cipher = match Aes256Gcm::new_from_slice(key) {
            Ok(c) => c,
            Err(e) => return Err(CryptoError::InvalidKey(e.to_string())),
        };

        // convertimos el IV a Nonce
        let nonce = Nonce::from_slice(iv);

        // cifrado
        let ciphertext = match cipher.encrypt(nonce, data) {
            Ok(c) => c,
            Err(e) => return Err(CryptoError::EncryptionFailed(e.to_string())),
        };
        Ok(ciphertext)
    }

    // descifra datos
    pub fn decrypt(key: &[u8; 32], iv: &[u8; 12], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // esto devuelve un Result asi que manjaremosel error
        let cipher = match Aes256Gcm::new_from_slice(key) {
            Ok(c) => c,
            Err(e) => return Err(CryptoError::InvalidKey(e.to_string())),
        };
        // porque el tamaño d e[u8; 12]
        // ese tamaño asi que podra tener nonce
        let nonce = Nonce::from_slice(iv);

        let plain_text = match cipher.decrypt(nonce, data) {
            Ok(c) => c,
            Err(e) => return Err(CryptoError::DecryptionFailed(e.to_string())),
        };
        Ok(plain_text)
    }
}
