use std::env; // Para leer variables de entorno (como la dirección de Redis).

// Esta estructura es un contenedor para el cliente de Redis.
// La usaremos para pasar el cliente a otras partes del programa que necesiten hablar con Redis.
#[derive(Clone)] // Permite clonar fácilmente el cliente para pasarlo a múltiples hilos o tareas.
pub struct RedisClient {
    pub client: redis::Client, // El cliente de Redis real, que gestiona la conexión.
}

impl RedisClient {
    // Función para crear una nueva instancia de nuestro cliente.
    pub async fn new() -> Self {
        // Leemos la URL de Redis de una variable de entorno. Si no existe, usamos la dirección local por defecto.
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        // Intentamos crear el cliente de Redis con esa URL. Si falla, el programa se detendrá con un mensaje de error.
        let client = redis::Client::open(redis_url)
            .expect("Fallo al conectar a Redis. ¿Está Redis instalado y corriendo?");

        println!("Cliente de Redis creado correctamente.");

        // Devolvemos nuestra estructura que contiene el cliente.
        RedisClient { client }
    }

    // Función de ayuda (helper) para obtener una conexión asíncrona del pool de conexiones.
    // Es como pedir una línea telefónica libre para hacer una llamada a Redis.
    pub async fn get_connection(&self) -> redis::RedisResult<redis::aio::MultiplexedConnection> {
        self.client.get_multiplexed_async_connection().await
    }
}

#[cfg(test)]
mod test {
    use redis::AsyncCommands;
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn rate_limit_window_resets_after_expiration() {
        // Conectar a Redis
        let client =
            redis::Client::open("redis://127.0.0.1/").expect("No se pudo crear el cliente Redis");

        let mut conn = client
            .get_multiplexed_async_connection()
            .await
            .expect("No se pudo conectar a Redis");

        let key = "rate_limit:ip:test_ip";

        // Limpiar por si existe previamente
        let _: () = conn.del(key).await.unwrap();

        // Simular contador con TTL de 2 segundos
        let _: () = conn.set_ex(key, 5, 2).await.unwrap();

        // Verificar que el TTL está activo
        let ttl: i64 = conn.ttl(key).await.unwrap();
        assert!(ttl > 0, "El TTL debería estar activo");

        // Esperar a que expire
        sleep(Duration::from_secs(3)).await;

        // Verificar que la key ya no existe
        let exists: bool = conn.exists(key).await.unwrap();
        assert!(
            !exists,
            "El contador debería haber expirado después del TTL"
        );
    }
}
