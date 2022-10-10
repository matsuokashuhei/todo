use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize)]
struct JWKs {
    keys: Vec<RSAKey>,
}

#[derive(Debug, Deserialize, Clone)]
struct RSAKey {
    kid: String,
    alg: String,
    n: String,
    e: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    pub sub: String, // Optional. Subject (whom token refers to)
}

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
        .and_then(|header| header.to_str().ok())
        .unwrap();
    // if let Some(authorization) = authorization {
    //     let claims = verify_token(authorization).await.unwrap();
    // }
    let token = verify_token(authorization).await.unwrap();
    req.extensions_mut().insert(token.claims);
    Ok(next.run(req).await)
}

async fn verify_token(token: &str) -> Result<TokenData<Claims>, Box<dyn Error>> {
    // https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
    let key = fetch_key(token).await.unwrap();
    let components = &DecodingKey::from_rsa_components(&key.n, &key.e).unwrap();
    let validation = &Validation::new(Algorithm::RS256);
    let token = decode::<Claims>(token, components, validation).unwrap();
    println!("token: {:?}", token);
    Ok(token)
}

async fn fetch_key(token: &str) -> Result<RSAKey, Box<dyn Error>> {
    // Vaildating a JSON web token
    // https://docs.aws.amazon.com/ja_jp/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-verifying-a-jwt.html
    //
    // Code examples
    // - https://github.com/rib/jsonwebtokens-cognito/blob/master/src/lib.rs
    // - https://github.com/Keats/jsonwebtoken
    let kid = decode_header(token).unwrap().kid.unwrap();
    println!("kid: {:?}", kid);
    let url = format!(
        "https://cognito-idp.{}.amazonaws.com/{}/.well-known/jwks.json",
        std::env::var("AWS_REGION").unwrap(),
        std::env::var("AWS_COGNITO_USER_POOL_ID").unwrap(),
    );
    let response: reqwest::Response = reqwest::get(url).await?;
    let jwks: JWKs = response.json().await?;
    println!("jwks: {:?}", jwks);
    let key = jwks
        .keys
        .into_iter()
        .find(|key| key.alg == "RS256" && key.kid == kid)
        .unwrap();
    println!("key: {:?}", key);
    Ok(key)
    // let key = &DecodingKey::from_rsa_components(&key.n, &key.e).unwrap();
    // Ok(key)
}
