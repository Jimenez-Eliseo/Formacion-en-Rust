#[test]
fn test_fail_fast_no_db_connection() {
    use safe_config::start_microservice;
    use std::env;
    // Configuración inválida (password débil)
    unsafe {
        env::set_var("POSTGRES_USER", "admin");
        env::set_var("POSTGRES_PASSWORD", "123456"); // débil
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_PORT", "5432");
        env::remove_var("PATH_SECRETS"); // forzar uso de env
    }

    let mut connection = false;

    let result = start_microservice(&mut connection);

    // Debe fallar la configuración
    assert!(result.is_err());

    // Y NO debe intentar conectarse
    assert!(!connection);
}
