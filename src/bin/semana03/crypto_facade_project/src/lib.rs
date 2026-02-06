mod internal;

use internal::engine::CipherEngine;
use internal::formatter::FormatHandler;
use internal::key_derivation::KeyDerivation;

use crate::internal::error_handler::FacadeError;

/// Fachada
/// Es la ÚNICA interfaz pública
pub struct SimpleEncryptor;

impl SimpleEncryptor {
    // interfaz simple
    pub fn encrypt_message(password: &str, message: &str) -> Result<String, FacadeError> {
        // password a Key
        let key = KeyDerivation::derive_key(password);

        // 2. IV automático
        let iv = CipherEngine::generate_iv();

        // 3. Cifrado real
        let ciphertext = CipherEngine::encrypt(&key, &iv, message.as_bytes())?;

        // 4. Empaquetado
        Ok(FormatHandler::pack(&iv, &ciphertext))
    }

    /// Descifrado usando la misma fachada
    pub fn decrypt_message(password: &str, data: &str) -> Result<String, FacadeError> {
        // 1. Password → Key
        let key = KeyDerivation::derive_key(password);

        // 2. Desempaquetado
        let (iv, cipher) = FormatHandler::unpack(data)?;

        // 3. Descifrado
        let plain = CipherEngine::decrypt(&key, &iv, &cipher)?;

        // 4. Bytes → texto
        //String::from_utf8(plain).unwrap()?

        let text = match String::from_utf8(plain) {
            Ok(text) => text,
            Err(_) => return Err(FacadeError::InvalidUtf8),
        };
        Ok(text)
    }
}
