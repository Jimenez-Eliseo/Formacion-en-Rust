use crypto_facade_project::SimpleEncryptor;

#[test]
fn integration_encrypt_decrypt_real_crypto() {
    let password = "1234";
    let message = "mensaje secreto";

    //  encrypt
    let encrypted = SimpleEncryptor::encrypt_message(password, message).expect("Error de cifrado");

    println!("--------------------------------------");
    println!("{}", encrypted);
    println!("--------------------------------------");

    // no debe contener texto plano
    assert!(!encrypted.contains(message));

    let decrypted =
        SimpleEncryptor::decrypt_message(password, &encrypted).expect("Error de descifrado");

    println!("######################################");
    println!("{}", decrypted);
    println!("######################################");

    assert_eq!(decrypted, message);
}
