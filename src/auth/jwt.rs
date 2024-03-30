use anyhow::{Context, Result};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: u64,
}

pub fn create_token(user_id: i64) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET can not be fetched.");

    let now: DateTime<Utc> = Utc::now();
    let now_epoch: i64 = now.timestamp();
    let delta: Option<TimeDelta> =
        Some(TimeDelta::try_minutes(5).expect("Could not get time delta!"));
    let exp: i64 = match delta {
        Some(exp) => exp.num_seconds() + now_epoch,
        None => 0,
    };

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp as u64,
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
/*
pub async fn validate_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match decode_token(credentials.token()) {
        Ok(_claims) => Ok(req),
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}
*/
