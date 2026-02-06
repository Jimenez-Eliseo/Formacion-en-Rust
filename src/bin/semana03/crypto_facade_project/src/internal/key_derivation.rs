use sha2::{Digest, Sha256};

pub struct KeyDerivation;

impl KeyDerivation {
    // genera una clave de 32 bytes
    // esto permite que AES-256 pueda usarla
    // esto lo hacemos a partir de un password en texto
    pub fn derive_key(password: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        // convertimos texto a bytes
        // como los algoritmos criptograficos trabajanen bytes
        // por eso le pasamos password.as_bytes()
        hasher.update(password.as_bytes());

        // obtenemos el hash criptografico
        // a partir de el password
        let result = hasher.finalize();

        // convertimos a array fijo
        // y asi poder retornar de la misma manera
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        key
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_mismo_password() {
        let password = "1234";

        let key1 = KeyDerivation::derive_key(password);
        let key2 = KeyDerivation::derive_key(password);

        assert_eq!(key1, key2);
    }

    #[test]
    fn passwords_diferentes_generan_keys_diferentes() {
        let key1 = KeyDerivation::derive_key("1234");
        let key2 = KeyDerivation::derive_key("abcd");

        assert_ne!(key1, key2);
    }
}
