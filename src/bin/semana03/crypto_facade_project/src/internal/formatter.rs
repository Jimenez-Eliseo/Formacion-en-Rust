use base64::{Engine as _, engine::general_purpose};

use crate::internal::error_handler::PackError;

pub struct FormatHandler;

impl FormatHandler {
    // Empaqueta IV + ciphertext en un String seguro
    pub fn pack(iv: &[u8; 12], ciphertext: &[u8]) -> String {
        let mut combined = Vec::new();
        // [ IV | CIPHERTEXT (en bianrio) ]
        combined.extend_from_slice(iv);
        combined.extend_from_slice(ciphertext);

        // binario a texto
        // pasa de binario con base64 a texto
        general_purpose::STANDARD.encode(combined)
    }

    /// Desempaqueta el string
    pub fn unpack(data: &str) -> Result<([u8; 12], Vec<u8>), PackError> {
        // recibimos un string codificaod en base64 ahora lo volvemos a binario
        let decoded = match general_purpose::STANDARD.decode(data) {
            Ok(data) => data,
            Err(e) => return Err(PackError::UnPack(e.to_string())),
        };

        // hacemos un slice los 12 primerossquele pertenecen a iv
        let (iv_bytes, cipher) = decoded.split_at(12);

        // ponemos el tammaño fijoque retornaremosquees de [u8; 12]
        let mut iv = [0u8; 12];
        // copiamos de un slice
        iv.copy_from_slice(iv_bytes);

        Ok((iv, cipher.to_vec()))
    }
}
