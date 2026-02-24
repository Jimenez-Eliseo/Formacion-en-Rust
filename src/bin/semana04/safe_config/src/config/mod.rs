pub mod models;
pub mod validador;

use dotenvy::dotenv;
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

use crate::config::models::DatabaseConfig;
use crate::config::validador::validate_database_config;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing configuration: {0}")]
    Missing(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("IO error")]
    Io(#[from] std::io::Error),
}

fn read_secret_or_env(key: &str) -> Result<String, ConfigError> {
    // cargamos el .env por si acaso estamos trabajando
    dotenv().ok();

    // ruta donde tenemos los secretos
    let path_secrets =
        env::var("PATH_SECRETS").map_err(|_| ConfigError::Missing("Path secrets".into()))?;

    // creamos la ruta a el archivo
    let secret_path = format!("/{}/{}", path_secrets, key);

    // verificamos que exista
    if Path::new(&secret_path).exists() {
        let value = fs::read_to_string(secret_path)?;
        return Ok(value.trim().to_string());
    }

    //buscamso la variable
    env::var(key).map_err(|_| ConfigError::Missing(key.into()))
}

pub fn load_database_config() -> Result<DatabaseConfig, ConfigError> {
    // recolectmao las variables y si hay algun error se lanza y se detiene
    let user = read_secret_or_env("POSTGRES_USER")?;
    let password = read_secret_or_env("POSTGRES_PASSWORD")?;
    let host = read_secret_or_env("POSTGRES_HOST")?;

    // para el puerto parsemos y paramos en caso de error
    let port: u16 = read_secret_or_env("POSTGRES_PORT")?
        .parse()
        .map_err(|_| ConfigError::Missing("POSTGRES_PORT invalid".into()))?;

    let config = DatabaseConfig {
        user,
        password,
        host,
        port,
    };

    // se lepasa lo datos de configuracion ya obtenidos
    validate_database_config(&config).map_err(|e| ConfigError::Validation(e.to_string()))?;

    Ok(config)
}

#[cfg(test)]
mod test {

    use crate::*;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn password_fail() {
        let result = load_database_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_docker_secrets_loading() {
        let dir = tempdir().unwrap();

        // crear archivos simulando Docker Secrets
        fs::write(dir.path().join("POSTGRES_USER"), "admin").unwrap();
        fs::write(
            dir.path().join("POSTGRES_PASSWORD"),
            "super_secure_password",
        )
        .unwrap();
        fs::write(dir.path().join("POSTGRES_HOST"), "localhost").unwrap();
        fs::write(dir.path().join("POSTGRES_PORT"), "5432").unwrap();

        // apuntar PATH_SECRETS al directorio temporal
        unsafe {
            env::set_var("PATH_SECRETS", dir.path());
        }

        let cfg = load_database_config().unwrap();

        assert_eq!(cfg.user, "admin");
        assert_eq!(cfg.password, "super_secure_password");
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 5432);
    }
}
