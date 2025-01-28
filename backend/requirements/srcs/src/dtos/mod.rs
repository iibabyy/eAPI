use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::ValidationError;

pub mod user;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Success,
    Failure,
    Error,
    Pending,
}


pub fn validate_password(password: &str) -> Result<(), ValidationError> {
	if password.is_empty() {
		return Err(ValidationError::new("failed").with_message(Cow::Borrowed("Password is required")))
	}

	if password.len() < 6 {
		return Err(ValidationError::new("failed").with_message(Cow::Borrowed("Password must be at least 6 characters")))
	}

	if password.len() > 25 {
		return Err(ValidationError::new("failed").with_message(Cow::Borrowed("Password must be at most 25 characters")))
	}

	return Ok(())
}
