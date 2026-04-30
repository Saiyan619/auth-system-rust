use std::{env, sync::Arc};

use axum::{Extension, Json, Router, extract::Query, http::{HeaderMap, StatusCode, header::{self, SET_COOKIE}}, response::{IntoResponse, Redirect}, routing::{get, post}};
use axum_extra::extract::cookie::Cookie;
use chrono::{Duration, Utc};
use validator::Validate;

use crate::{AppState, db::UserExt, dtos::{ForgotPasswordRequestDto, LoginUserRequestDto, RegisterUserDto, ResetPasswordRequestDto, Response, UserLoginResponseDto, VerifyEmailQueryDto}, errors::{ErrorMessage, HttpError}, mail::mails::{send_forgot_password_email, send_verification_email}, utils::{password::{self, compare}, token}};


pub fn auth_handler() -> Router {
    Router::new()
    .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify", get(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}

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

pub async fn login(Extension(app_state): Extension<Arc<AppState>>, Json(body): Json<LoginUserRequestDto>) 
-> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::server_error(e.to_string()))?;
    let result = app_state.db_client.get_user(None, None, Some(&body.email), None)
    .await
    .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    let password = compare(&body.password, &user.password)
    .map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    if password {
        let token = token::create_token(&user.id.to_string(), &app_state.env.jwt_secret.as_bytes(), app_state.env.jwt_maxage)
        .map_err(|e| HttpError::server_error(e.to_string()))?;
         
         let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
         let cookie = Cookie::build(("token", token.clone())).path("/").max_age(cookie_duration).http_only(true).build();

         let response = axum::response::Json(UserLoginResponseDto{
            status: "success".to_string(),
            token: token.to_string()
         });

         let mut header = HeaderMap::new();

         header.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

         let mut response = response.into_response();

         response.headers_mut().extend(header);

         Ok(response)
         }else {
             Err(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))
         }
    }


    pub async fn verify_email(Query(query_param): Query<VerifyEmailQueryDto>,Extension(app_state): Extension<Arc<AppState>>) 
    -> Result<impl IntoResponse, HttpError>{
        query_param.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
        let result = app_state.db_client.get_user(None, None, None, Some(&query_param.token))
        .await.map_err(|_| HttpError::bad_request(ErrorMessage::TokenNotProvided.to_string()))?;
       let user = result.ok_or(HttpError::bad_request(ErrorMessage::InvalidToken.to_string()))?;

       if let Some(expires_at) = user.token_expires_at{
        if Utc::now() > expires_at {
            Err(HttpError::server_error(ErrorMessage::InvalidToken.to_string()))?
        }else {
            Err(HttpError::server_error(ErrorMessage::PermissionDenied.to_string()))?
        }
       }

       app_state.db_client.verified_token(&query_param.token).await.map_err(|e| HttpError::bad_request(e.to_string()))?;

       let send_email_verification = send_verification_email(&user.email, &user.name, &query_param.token).await;

       if let Err(e) = send_email_verification{
        eprintln!("Error: Could not send email verification: {}", e)
       }

       let token = token::create_token(&user.id.to_string(), app_state.env.jwt_secret.as_bytes(), app_state.env.jwt_maxage).map_err(|e| HttpError::server_error(e.to_string()))?;

       let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
       let cookie = Cookie::build(("token", token.clone())).path("/").max_age(cookie_duration).http_only(true).build();

       let mut headers = HeaderMap::new();
       headers.append(header::SET_COOKIE, token.to_string().parse().unwrap());

       let frontend_url = format!("http://localhost:5173/settings");
       let redirect = Redirect::to(&frontend_url);

       let mut response = redirect.into_response();
       response.headers_mut().extend(headers);

       Ok(response)
    }
  
pub async fn forgot_password(Extension(app_state): Extension<Arc<AppState>>, Json(body): Json<ForgotPasswordRequestDto>)
 -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let result = app_state.db_client.get_user(None, None, Some(&body.email), None).await.map_err(|e| HttpError::server_error(e.to_string()))?;
    let user = result.ok_or(HttpError::bad_request("Email not found!".to_string()))?;
    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::minutes(30);
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    app_state.db_client.add_verified_token(user_id, &verification_token, expires_at).await.map_err(|e| HttpError::server_error(e.to_string()))?;

    let reset_link = format!("http://localhost:5173/reset-password?token={}", &verification_token);

    let email_sent = send_forgot_password_email(&user.email, &user.name, &reset_link).await;

    if let Err(e) = email_sent  {
        eprint!("Email failed to send!");
        return Err(HttpError::server_error("Failed to send forgot password reset email"));
    }

    let response = Response{
        status: "success",
        message: "Password reset link has been sent to your email.".to_string(),
    };

    Ok(Json(response))
}

pub async fn reset_password(Extension(app_state): Extension<Arc<AppState>>, Json(body): Json<ResetPasswordRequestDto>) 
-> Result<impl IntoResponse, HttpError>{
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let result = app_state.db_client.get_user(None, None, None, Some(&body.token.to_string()))
    .await.map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request("Failed to get user!".to_string()))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
             return Err(HttpError::bad_request("Verification token has expired".to_string()))?;
        }else {
            return Err(HttpError::bad_request("Invalid verification token".to_string()))?;
        }
    }

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let hashed_password = password::hash(&body.new_password).map_err(|e| HttpError::server_error(e.to_string()))?;


     app_state.db_client
        .update_user_password(user_id.clone(), hashed_password)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state.db_client
        .verified_token(&body.token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        message: "Password has been successfully reset.".to_string(),
        status: "success",
    };

    Ok(Json(response))

}

