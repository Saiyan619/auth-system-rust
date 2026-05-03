use std::sync::Arc;

use axum::{Extension, extract::Request, http::{StatusCode, header}, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{AppState, db::UserExt, errors::{ErrorMessage, HttpError}, models::{User, UserRole}, utils::token};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWTAuthMiddeware {
    pub user: User
}

pub async fn auth(cookie: CookieJar, Extension(appstate):Extension<Arc<AppState>>, mut req: Request, next:Next) -> Result<impl IntoResponse, HttpError>{
    // find token in cookie if not found fallback to auth header
    let cookies = cookie.get("token").map(|token| token.value().to_string())
    .or_else(|| {
        req.headers().
        get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok() )
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer "){
                Some(auth_value[7..].to_owned())
            }else {
                None
            }
        })
    });

    let token = cookies.ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::TokenNotProvided.to_string())
    })?;

    let token_details = match token::decode_token(token, appstate.env.jwt_secret.as_bytes()){
        Ok(token_details) => token_details,
        Err(_) => {
            return Err(HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))
        }
    };

    let user_id = uuid::Uuid::parse_str(&token_details.to_string()).map_err(|_| {
        HttpError::unauthorized(ErrorMessage::InvalidToken.to_string())
    })?;

    let user = appstate.db_client.get_user(Some(user_id), None, None, None)
    .await
    .map_err(|_| {
        HttpError::unauthorized(ErrorMessage::UserNoLongerExist.to_string())
    })?;

    let user = user.ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::UserNoLongerExist.to_string())
    })?;

    req.extensions_mut().insert(JWTAuthMiddeware {
        user:user.clone()
    });

    Ok(next.run(req).await)

}


pub async fn role_check(
    Extension(_app_state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
    required_role: Vec<UserRole>
) -> Result<impl IntoResponse, HttpError>{
    let user = req.extensions().get::<JWTAuthMiddeware>().ok_or_else(||{
        HttpError::unauthorized(ErrorMessage::UserNotAuthenticated.to_string())
    })?;

    if !required_role.contains(&user.user.role){
        return Err(HttpError::new(ErrorMessage::PermissionDenied.to_string(), StatusCode::FORBIDDEN));
    }

    Ok(next.run(req).await)
}