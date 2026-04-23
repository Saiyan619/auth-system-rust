use axum::{Json, http::{self, StatusCode}};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorMessage, HttpError};

#[derive(Serialize, Deserialize)]
pub struct TokenClaim{
    sub: String,
    iat: usize,
    exp: usize
}

pub fn create_token(user_id: &str, secret: &[u8], expires_in_seconds:i64) -> Result<String, jsonwebtoken::errors::Error>{
    //check if user.id is empty if it is return a jsonweb error
// get the current time
// timestamp the current time 
// get the exp 
// return the struct instance(claim)
// encode jwt token
    if user_id.is_empty(){
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into())
    }
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(expires_in_seconds)).timestamp() as usize;

    let claims = TokenClaim{
        sub: user_id.to_string(),
        iat,
        exp
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))

}


pub fn decode_token<T:Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError>{
    let decode = decode::<TokenClaim>(&token.into(), &DecodingKey::from_secret(secret), &Validation::new(Algorithm::HS256));

    match decode {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(HttpError::new(ErrorMessage::InvalidToken.to_string(), StatusCode::UNAUTHORIZED))
    }
}