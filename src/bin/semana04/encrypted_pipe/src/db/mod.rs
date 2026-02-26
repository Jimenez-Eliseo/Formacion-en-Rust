pub mod tls_config;

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{path::Path, time::Duration};

pub use tls_config::TlsError;

pub async fn create_secure_tls(
    database_url: &str,
    ca_cert_path: &Path,
    max_connections: u32,
) -> Result<(PgPool, bool), TlsError> {
    // usamos la funcion que hace la configuracion tls
    let connect_options = tls_config::configure_tls(database_url, ca_cert_path)?;

    // creamos las options del pool
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(connect_options)
        .await?; 

    // probamos que es seguro o no
    let is_secure = tls_config::test_secure_connection(&pool).await?;

    Ok((pool, is_secure))
}

#[cfg(test)]
mod tests {

    use crate::create_secure_tls;
    use std::env;
    use std::path::Path;

    #[tokio::test]
    async fn pool_is_secure() {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");

        let ca_cert = env::var("CA_CERT_PATH").expect("CA_CERT_PATH missing");

        let (_pool, is_secure) = create_secure_tls(&database_url, Path::new(&ca_cert), 5)
            .await
            .expect("Pool creation failed");

        assert!(is_secure);
    }
}

