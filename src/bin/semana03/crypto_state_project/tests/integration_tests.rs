use crypto_state_project::SecureChannel;

#[test]
fn test_flujo_completo() {
    // Crear canal (Uninitialized)
    let mut channel = SecureChannel::new();
    println!("Canal creado. Estado: {}", channel.current_state());

    // Intentar enviar datos sin handshake
    let resultado = channel.send_data("datos importantes");
    println!("Intento de envío sin handshake: {:?}", resultado);
    assert!(resultado.is_err());

    // Iniciar handshake
    println!("\n--- Iniciando handshake ---");
    let handshake = channel.start_handshake();
    println!("Resultado: {:?}", handshake);
    assert!(handshake.is_ok());
    println!("Estado actual: {}", channel.current_state());

    // Intentar enviar durante handshake
    let resultado = channel.send_data("datos durante handshake");
    println!("Intento de envío durante handshake: {:?}", resultado);
    assert!(resultado.is_err());

    // 3. Completar handshake
    println!("\n--- Completando handshake ---");
    let completado = channel.complete_handshake();
    println!("Resultado: {:?}", completado);
    assert!(completado.is_ok());
    println!("Estado actual: {}", channel.current_state());

    // 4. Enviar datos normalmente
    println!("\n--- Enviando datos ---");
    let envio = channel.send_data("HOLA MUNDO SECRETO");
    println!("Resultado del envío: {:?}", envio);
    assert!(envio.is_ok());

    // 5. Cerrar canal
    println!("\n--- Cerrando canal ---");
    let cerrado = channel.close();
    println!("Resultado: {:?}", cerrado);
    assert!(cerrado.is_ok());
    println!("Estado actual: {}", channel.current_state());

    // 6. Intentar enviar después de cerrar
    println!("\n--- Intentando enviar después de cerrar ---");
    let envio_final = channel.send_data("otro mensaje");
    println!("Resultado: {:?}", envio_final);
    assert!(envio_final.is_err());
}

#[test]
fn test_errores_especificos() {
    let mut channel = SecureChannel::new();

    // Probar error específico de Uninitialized
    match channel.send_data("test") {
        Err(msg) => assert!(msg.contains("sin completar el handshake")),
        Ok(_) => panic!("Debería haber fallado"),
    }

    // Avanzar a Handshake
    channel.start_handshake().unwrap();

    // Probar error específico de Handshake
    match channel.send_data("test") {
        Err(msg) => assert!(msg.contains("Handshake en progreso")),
        Ok(_) => panic!("Debería haber fallado"),
    }
}
