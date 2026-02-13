use crate::SecretKey;

/// Simulación de una operación criptográfica.
/// Toma ownership de la clave.
pub fn use_key(key: SecretKey) -> usize {
    // Simulación: retornamos el tamaño
    key.as_bytes().len()

    // Al terminar esta función,
    // `key` sale de scope → se ejecuta Drop automáticamente.
}
