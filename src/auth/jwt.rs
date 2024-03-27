use actix_web::cookie::time::Duration;
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, encode, Algorithm, Claims, DecodingKey, EncodingKey, Header, Validation,
};
use std::env;

fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET can not be fetched.");

    let now = chrono::Utc::now();
    let exp = now + Duration::minutes(5).num_miliseconds();
    match expiration {
        Ok(Expiration) => Ok(Expiration),
        Err(e) => println!("An error occurred: {}", e),
    }

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}

fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET can not be fetched.");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}

async fn validate_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match decode_token(credentials.token()) {
        Ok(_claims) => Ok(req),
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}
