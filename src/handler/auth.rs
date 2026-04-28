use std::sync::Arc;

use axum::{Extension, Json, body, http::{self, StatusCode}, response::IntoResponse};
use chrono::{Duration, Utc};
use validator::Validate;

use crate::{AppState, db::UserExt, dtos::{RegisterUserDto, Response}, errors::{ErrorMessage, HttpError}, mail::mails::send_verification_email, utils::password};



pub async fn register(Extension(app_state):Extension<Arc<AppState>>, Json(body): Json<RegisterUserDto>)
-> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|err| HttpError::bad_request(err.to_string()))?;
    
    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);

    let hashed_password = password::hash(&body.password).map_err(|err| HttpError::server_error(err.to_string()))?;

    let result = app_state.db_client.save_user(&body.name, &body.email, &hashed_password, &verification_token, expires_at).await;

    match result {
        Ok(_user) => {
            let send_email_result = send_verification_email(&body.email, &body.name, &verification_token).await;
            if let Err(e) = send_email_result{
                eprintln!("Error sending email Verification: {e}");
            }
            Ok((StatusCode::CREATED, Json(Response{
                status: "success",
                message: "Registration successful! Please check your email to verify your account.".to_string()
            })))
        },
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation(){
                Err(HttpError::unique_constraint_violation(ErrorMessage::EmailExist.to_string()))
            }else{
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string()))
    }
}