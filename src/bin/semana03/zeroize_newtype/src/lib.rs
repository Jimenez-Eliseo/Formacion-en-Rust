pub mod crypto_ops;

pub struct SecretKey(Vec<u8>);

impl SecretKey {
    pub fn new(key: Vec<u8>) -> Self {
        SecretKey(key)
    }

    // devolvemos una slice de referencia
    // de los bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for SecretKey {
    // como esta en la tarea al pasar esta
    // referencia mutable no solo cambiamos el valor
    fn drop(&mut self) {
        // para la iteracion hacemos el recorrio
        // dandoke la referancia mutable
        // antes de que se mueva nosotros solapamos
        // con 0 y esot como lo mueve
        // esto fisicamente estadisponiblepeor la
        // variable que hayausado esta su puntro y demas como  0
        // osea que va implentar de uevo
        // por eso esque fisicamente existe hastaquesesolape
        // solapar con 0 es una tecnica basica pero sirve
        for byte in &mut self.0 {
            // byte solo es una referancia y
            // para acceder al valor real
            // apuntamos de esa manera con *
            // delante de la referencia byte
            *byte = 0;
        }
    }
    // despues de esto se livera memoria
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_accessibility() {
        let raw_data = vec![10, 20, 30, 40];

        // aqui estamos moviendo raw_data.clone()
        // hacia el constructor de SecretKey
        let key = SecretKey::new(raw_data.clone());

        // Validamos que los datos sean accesibles mientras la variable "viva"
        // mientras key no salga del scope, podemos acceder a los datos       // Validamos que los datos sean accesibles mientras la variable "viva"
        assert_eq!(key.as_bytes(), &raw_data[..]);

        // tambien verificamos que la longitud sea correcta
        assert_eq!(key.as_bytes().len(), 4);
    }
}
