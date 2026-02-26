mod db;

use db::create_secure_tls;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar definida");

    let ca_cert_path = env::var("CA_CERT_PATH").unwrap_or_else(|_| "./certs/ca.crt".to_string());
    println!("->>>>>>>> {}", ca_cert_path);
    let ruta = Path::new(&ca_cert_path);
    if ruta.exists() {
        println!("SI EXISTE");
    }

    let max_connections = env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(5);

    let (_pool, is_secure) =
        create_secure_tls(&database_url, Path::new(&ca_cert_path), max_connections)
            .await
            .expect("Error inicializando conexión segura");

    println!("Pool inicializado y es seguro?: {}", is_secure);
}
