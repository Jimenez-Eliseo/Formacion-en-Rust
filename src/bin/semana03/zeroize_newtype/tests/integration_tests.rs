use zeroize_newtype::{SecretKey, crypto_ops::use_key};

#[test]
fn demostracion_de_error() {
    let mi_clave = SecretKey::new(vec![1, 2, 3]);

    let tamano = use_key(mi_clave); // La clave "se mueve" aquí dentro.

    println!("tamano de vec {}", tamano);

    // comentamos la linea porque
    // nos dice el error que mi_clave
    // a sido movido
    // let _ = use_key(mi_clave);
}
