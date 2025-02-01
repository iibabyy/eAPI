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

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
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
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
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
    pub sold: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            sold: user.sold,
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: Status,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForeignUserResponseDto {
    pub status: Status,
    pub data: FilterForeignUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: Status,
    pub users: Vec<FilterForeignUserDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub status: Status,
    pub data: UserData,
    pub token: String,
}
