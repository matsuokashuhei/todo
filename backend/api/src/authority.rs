use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::{error::Error, sync::Arc};

pub async fn authenticate_user<B>(
    mut req: Request<B>,
    next: Next<B>,
    // database: Database,
) -> Result<Response, StatusCode> {
    // note
    // https://docs.rs/axum/latest/axum/middleware/index.html#sharing-state-between-handlers-and-middleware
    let authorization = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    if let Some(authorization) = authorization {
        verify_token(authorization);
    }
    Ok(next.run(req).await)
}

async fn verify_token(token: &str) -> Result<String, Box<dyn Error>> {
    Ok(String::from(token))
}
