use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::{models::User, utils::status::{validate_password, Status}};


#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    #[validate(custom(function="validate_password"))]
    pub password: String,

    #[validate(custom(function="validate_password"))]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

	#[validate(custom(function="validate_password"))]
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterForeignUserDto {
    pub name: String,
    pub email: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterForeignUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterForeignUserDto {
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub fn filter_users(users: &[User]) -> Vec<FilterForeignUserDto> {
        users.iter().map(|user| FilterForeignUserDto::filter_user(user)).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub sold_in_cents: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            sold_in_cents: user.sold_in_cents,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: Status,
    pub data: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForeignUserResponseDto {
    pub status: Status,
    pub data: FilterForeignUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: Status,
    pub data: Vec<FilterForeignUserDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub status: Status,
    pub data: FilterUserDto,
    pub token: String,
}
