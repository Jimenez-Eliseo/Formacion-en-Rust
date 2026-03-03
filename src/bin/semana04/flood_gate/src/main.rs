mod middleware;
mod store;

use middleware::limiter::rate_limit_middleware;
use store::redis_client::RedisClient;

use axum::{Router, routing::get};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

async fn root_handler() -> &'static str {
    "Hola Rate limit funcionando"
}

#[tokio::main]
async fn main() {
    let redis_client = Arc::new(RedisClient::new().await);

    let app = Router::new()
        .route("/", get(root_handler))
        .layer(axum::middleware::from_fn_with_state(
            redis_client.clone(),
            rate_limit_middleware,
        ))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor corriendo en http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

