use anyhow::{Context, Result};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

use actix_web::{dev, error, http, web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::{ready, Ready};

use crate::utils::error::{AppError, AppErrorType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

impl FromRequest for Claims {
    type Error = AppError;
    type Future = Ready<Result<Claims, AppError>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET can not be fetched.");
        if let Some(cookie) = req.cookie("auth_token") {
            let token = cookie.value();
            let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
            match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)) {
                Ok(data) => ready(Ok(data.claims)),
                Err(_) => ready(Err(AppError::from(AppErrorType::IncorrectLogin))),
            }
        } else {
            ready(Err(AppError::from(AppErrorType::IncorrectLogin)))
        }
    }
}

pub fn create_token(username: String) -> Result<String, jsonwebtoken::errors::Error> {
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
        sub: username,
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
