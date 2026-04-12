use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole{
    Admin,
    User
}
impl UserRole{
    pub fn to_str(&self) -> &str {
        match self{
            UserRole::Admin => "admin",
            UserRole::User => "user"
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, sqlx::FromRow)]
pub struct User{
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub verified: Option<String>,
    pub verification_token: Option<DateTime<Utc>>,
    pub role: UserRole,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}