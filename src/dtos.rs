use core::str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto{
    #[validate(length(min=1, message="Name is required"))]
    pub name: String,
    #[validate(length(min=1, message="Email is required"), email(message="Email is invalid"))]
    pub email: String,
    #[validate(length(min=6, message="Password is required and Password must be at least 6 characters"))]
    pub password: String,
    #[serde(rename = "passwordConfirm")]    
    pub password_confirm: String
}

#[derive(Validate, Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoginUserRequestDto{
    #[validate(length(min=6, message="Email is required"), email(message="Email is invalid"))]
    pub email:String,
    #[validate(length(min=6, message="Password is required"))]
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterUserDto{
    pub id: String,
    pub name: String,
    pub email: String,
    pub verified: bool,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>
}

impl FilterUserDto{
    pub fn filter_user(user:&User) -> Self{
     FilterUserDto{
    id: user.id.to_string(),
     name: user.name.to_string(),
      email: user.email.to_string(),
     verified: user.verified,
     role: user.role.to_str().to_string(),
    created_at: user.created_at.unwrap(),
    updated_at: user.updated_at.unwrap()
}
}

     pub fn filter_users(user: &[User]) -> Vec<FilterUserDto>{
        user.iter().map(FilterUserDto::filter_user).collect()
     }
    
}

#[derive(Debug, Validate, Serialize, Deserialize, Clone, Copy)]
pub struct UserPageQuery{
    #[validate(range(min=1))]
    pub page:Option<usize>,
    #[validate(range(min=1, max=50))]
    pub limit:Option<usize>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData{
    data: FilterUserDto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseTo{
    pub status: i32,
    pub data: UserData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponseTo{
    pub status: i32,
    pub data: Vec<UserData>,
    pub result: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RoleUpdateDto {
    #[validate(custom = "validate_user_role")]
    pub role: UserRole,
}

fn validate_user_role(role:&UserRole) -> Result<(), validator::ValidationError>{
    match role{
        UserRole::Admin | UserRole::User => Ok(()),
        _=>Err(validator::ValidationError::new("Invalid role"))
    }
}


#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "New password is required."),
        length(min = 6, message = "new password must be at least 6 characters")
    )]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "New password confirm is required."),
        length(min = 6, message = "new password confirm must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(
        length(min = 1, message = "Old password is required."),
        length(min = 6, message = "Old password must be at least 6 characters")
    )]
    pub old_password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required."),)]
    pub token: String,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
pub struct ForgotPasswordRequestDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "Token is required."),)]
    pub token: String,

    #[validate(
        length(min = 1, message = "New password is required."),
        length(min = 6, message = "new password must be at least 6 characters")
    )]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "New password confirm is required."),
        length(min = 6, message = "new password confirm must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,
}
