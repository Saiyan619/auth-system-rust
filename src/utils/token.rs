use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct TokenClaim{
    sub: String,
    iat: usize,
    exp: usize
}

pub fn create_token(user_id: &str, secret: &[u8], expires_in_seconds:i64){

}

pub fn decode_token(){

}