use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
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
        .and_then(|header| header.to_str().ok());
    if let Some(authorization) = authorization {
        let claims = verify_token(authorization).await.unwrap();
    }
    Ok(next.run(req).await)
}

async fn verify_token(token: &str) -> Result<TokenData<Claims>, Box<dyn Error>> {
    // https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
    // let contents = fs::read_to_string("https://cognito-idp.ap-northeast-1.amazonaws.com/ap-northeast-1_ZLVEczbUP/.well-known/jwks.json").expect("Should have been able to read the file");

    println!("token: {}", token);
    let body = get_jwks().await.unwrap();
    println!("body: {}", body);
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;
    Ok(token)
}

struct Keys {
    // {"keys":[{"alg":"RS256","e":"AQAB","kid":"aI83EO126zHrJfobLz3bfLmnRS9f5elwXVvc0fk+S+Y=","kty":"RSA","n":"xm9Gmhv2zYLB941LTc_ACcB5LoEiTSA7FaUrb2UmdzJnknCcIb1NRqbYv9_9gcUB0vr5FJGGu0hT43qfV7lAu6UtId8sFZ3gnnQlnLLcBJQ0_qagI8p2uobBHPBpuGvH110KeR_aeHu7vifVVuGA9RxXVfJad_0n76cSwcRwRRf-ZlPHIVK09cebY4fzXb67qIOv4rzcwOAd35lEeehQmhUU7q14FD04r9bUTPjKkqnPp5P19BYliD6zKpIEiXBD9Z6u4l73id2GnHaXLrIfPvme44WSsuPuBBwdwJmD0A8aKh5CW3zoiA_FIX88ML0BgWLFFHUzND-aoa2IiqDk2Q","use":"sig"},{"alg":"RS256","e":"AQAB","kid":"t2YZDTrME1gRqN7ar3J4pCTtST9zrPyS8K/inVlRnDE=","kty":"RSA","n":"2TPr_G086jIgvc98eeRDTAEr3omvOs3NhaYYAOmqm5CPa82I72LTfiFmZ_oFiczimhvTnCv0Fz1VyTtT_BdE7d4llngwIyvNbmCNqc7-KWLZw-V9lsPcJ1ckI_wz_jjTxEIYGEF-LQQNoM-wYtu2wbhCqF4hr3Gofkj2NiQEEEhrUXQoJ3nCnKDN8V6XZ0loPEGFr_UMTMXnKg7TBzxdMD8qRmBLskwW25Rc8dVUteeU7jEkX9IdJRXTJbs4UfxL8sSi5dI_Q066n7Po7bxKIE3bUnnPRRVQmLuAIHXPqajcAo0unQNfgFieskyvIAURObf8znxtujdUfQzz3X2i1Q","use":"sig"}]}
}
async fn get_jwks() -> Result<String, Box<dyn Error>> {
    let body = reqwest::get("https://cognito-idp.ap-northeast-1.amazonaws.com/ap-northeast-1_ZLVEczbUP/.well-known/jwks.json").await?.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    println!("body = {:?}", v);
    let keys = v
        .as_object()
        .and_then(|object| object.get_key_value("keys"));
    println!("keys = {:?}", keys);
    Ok(body)
}
