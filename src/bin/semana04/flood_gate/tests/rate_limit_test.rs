use std::process::Command;

#[test]
fn test_rate_limit_saturation_with_hey() {
    // Ejecuta hey para simular 100 requests con 20 concurrentes
    let output = Command::new("hey")
        .args(["-n", "100", "-c", "20", "http://localhost:3000/"])
        .output()
        .expect("Failed to execute hey");

    // Convertimos stdout a String
    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("{}", stdout);

    // Verificamos que haya respuestas 429
    assert!(
        stdout.contains("429"),
        "No se detectaron respuestas 429. El rate limit podría no estar funcionando."
    );
}

