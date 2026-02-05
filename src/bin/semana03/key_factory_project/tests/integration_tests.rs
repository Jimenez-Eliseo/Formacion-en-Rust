use key_factory_project::{SecurityLevel, factory::KeyFactory};

// se implemento que al generate_key se le
// tiene que pasar el SecurityLevel
// y de alli poder definir los 3 SecurityLevels
// y por detras el trait implementado en el archivo correspondiente
// tiene el valor espereado será "RSA-4096"
#[test]
fn test_high_level_rsa() {
    let key = KeyFactory::generate_key(SecurityLevel::High);
    assert_eq!(key.algorithm_name(), "RSA-4096");
}

// para esta prueba se se creo conel nivel de seguridad de Low
// quien contiene 16 bytes y este no tiene todas las posiciones
// en 0 si no que se definio aleatoriamente los elementos del array
// y contando los 0 que hay no tendriaque contener todos los 0s
#[test]
fn test_low_level_aes() {
    let key = KeyFactory::generate_key(SecurityLevel::Low);
    assert_eq!(key.as_bytes().len(), 16);
    let all_zero = key.as_bytes();
    let mut contador = 0;
    for valor in all_zero {
        if *valor == 0 {
            contador += 1;
        }
    }
    assert_ne!(contador, 16, "Todos son ceros");
}
