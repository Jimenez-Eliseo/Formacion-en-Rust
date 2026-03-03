use crate::store::redis_client::RedisClient;
use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{Request, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::{net::SocketAddr, sync::Arc};

const RATE_LIMIT_MAX: u32 = 10;
const RATE_LIMIT_WINDOW_SECONDS: usize = 60;
const PROGRESSIVE_BAN_THRESHOLD: u32 = 3;

pub async fn rate_limit_middleware(
    State(redis_client): State<Arc<RedisClient>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let client_ip = addr.ip().to_string();

    let request_key = format!("rate_limit:ip:{}", client_ip);
    let violation_key = format!("rate_limit:violations:{}", client_ip);
    let ban_key = format!("rate_limit:ban:{}", client_ip);

    let mut conn = redis_client
        .get_connection()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Redis no disponible").into_response())?;

    let is_banned: bool = redis::cmd("EXISTS")
        .arg(&ban_key)
        .query_async(&mut conn)
        .await
        .unwrap_or(false);

    if is_banned {
        let ttl: i64 = redis::cmd("TTL")
            .arg(&ban_key)
            .query_async(&mut conn)
            .await
            .unwrap_or(60);

        let mut response = (
            StatusCode::TOO_MANY_REQUESTS,
            "Baneado temporalmente por abuso",
        )
            .into_response();

        response
            .headers_mut()
            .insert(header::RETRY_AFTER, ttl.to_string().parse().unwrap());

        return Err(response);
    }

    let request_count: u32 = redis::cmd("INCR")
        .arg(&request_key)
        .query_async(&mut conn)
        .await
        .unwrap_or(1);

    if request_count == 1 {
        let _: () = redis::cmd("EXPIRE")
            .arg(&request_key)
            .arg(RATE_LIMIT_WINDOW_SECONDS)
            .query_async(&mut conn)
            .await
            .unwrap_or(());
    }

    if request_count > RATE_LIMIT_MAX {
        let retry_after: i64 = redis::cmd("TTL")
            .arg(&request_key)
            .query_async(&mut conn)
            .await
            .unwrap_or(RATE_LIMIT_WINDOW_SECONDS as i64);

        let mut response = (
            StatusCode::TOO_MANY_REQUESTS,
            "Límite de peticiones excedido",
        )
            .into_response();

        let headers = response.headers_mut();
        headers.insert(
            "X-RateLimit-Limit",
            RATE_LIMIT_MAX.to_string().parse().unwrap(),
        );
        headers.insert("X-RateLimit-Remaining", "0".parse().unwrap());
        headers.insert(
            header::RETRY_AFTER,
            retry_after.to_string().parse().unwrap(),
        );

        return Err(response);
    }

    let remaining = RATE_LIMIT_MAX - request_count;

    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert(
        "X-RateLimit-Limit",
        RATE_LIMIT_MAX.to_string().parse().unwrap(),
    );
    headers.insert(
        "X-RateLimit-Remaining",
        remaining.to_string().parse().unwrap(),
    );

    Ok(response)
}
