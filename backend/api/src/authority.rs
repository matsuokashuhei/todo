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

pub async fn verify_token<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let authorization = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    println!("authorization: {:?}", authorization);
    if let Some(authorization) = authorization {
        let result = decode_token(authorization).await;
        if let Ok(token) = result {
            req.extensions_mut().insert(Some(token.claims));
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        req.extensions_mut().insert(None as Option<Claims>);
        Ok(next.run(req).await)
    }
}

async fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    // https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
    let key = fetch_key(token).await.unwrap();
    let components = &DecodingKey::from_rsa_components(&key.n, &key.e).unwrap();
    let validation = &Validation::new(Algorithm::RS256);
    decode::<Claims>(token, components, validation)
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
        std::env::var("AWS_COGNITO_USER_POOL_ID")?,
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
