use crate::config::models::DatabaseConfig;

pub fn validate_database_config(cfg: &DatabaseConfig) -> Result<(), String> {
    // se quema datos solo para validar estas 2 contraseñas
    // se podriahber implementado un enum que retorna esas
    // cadenas que corresponden a contraseñas prohibidas
    let prohibido = ["postgres", "password"];

    if prohibido.contains(&cfg.password.as_str()) {
        return Err("Password inseguro, cambie a otro.".into());
    }

    // por se u16 que va hasta 65536
    // solo hay que validar el limite inferior
    if cfg.port < 1024 {
        return Err("El puerto esta fuera de rango permitido".into());
    }

    Ok(())
}
