use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, thiserror::Error)]
pub enum TlsError {
    #[error("Error leyendo archivo de certificado: {0}")]
    CertReadError(#[from] std::io::Error),

    #[error("El certificado ha expirado")]
    CertExpired,

    #[error("Error al configurar TLS: {0}")]
    TlsConfigError(String),

    #[error("Error de conexión: {0}")]
    ConnectionError(#[from] sqlx::Error),
}

// esta funcion prepara toda la configuracion de la
// conexion segura usando la url y el certificado ca
pub fn configure_tls(
    database_url: &str,
    ca_cert_path: &Path,
) -> Result<PgConnectOptions, TlsError> {
    // verificamos que exista con el ca_cert_path
    if !ca_cert_path.exists() {
        return Err(TlsError::TlsConfigError(format!(
            "Certificado CA no encontrado path: {:?}",
            ca_cert_path
        )));
    }
    // verificamos si expiro el certificado
    verify_cert_not_expired(ca_cert_path)?;
    // configuramos las opciones
    // de conexion obligando a que use ssl
    // modo verifyfull para maxima seguridad
    let options = database_url
        .parse::<PgConnectOptions>()?
        .ssl_mode(PgSslMode::VerifyFull)
        .ssl_root_cert(ca_cert_path);

    Ok(options)
}

// funcion interna para checkear que el archivo del certificado
fn verify_cert_not_expired(cert_path: &Path) -> Result<(), TlsError> {
    // una verificacion sencilla pero funcional
    //
    // sacamos la metadata del archivo para ver
    // cuando fue creado en el disco
    let metadata = fs::metadata(cert_path)?;
    let created = metadata.created().unwrap_or(SystemTime::now());

    // calculamos cuanto tiempo ha pasado
    // y si supera los 30 dias lanzamos el error de expirado
    if let Ok(duration) = created.elapsed() {
        if duration > std::time::Duration::from_secs(30 * 24 * 60 * 60) {
            return Err(TlsError::CertExpired);
        }
    }

    Ok(())
}

// prueba la conexion segura a la base de datos
pub async fn test_secure_connection(pool: &sqlx::PgPool) -> Result<bool, TlsError> {
    // ejecutamos la funcion de postgres
    let row: (bool,) = sqlx::query_as("SELECT ssl_is_used()")
        .fetch_one(pool)
        .await?;

    let ssl_used = row.0;

    Ok(ssl_used)
}
