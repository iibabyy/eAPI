use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::ValidationError;

pub mod user;


#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Success,
    Failure,
    Error,
    Pending,
}

impl Into<String> for Status {
	fn into(self) -> String {
		self.to_string()
	}
}

impl ToString for Status {
	fn to_string(&self) -> String {
		match self {
			Status::Success => "success",
			Status::Error => "error",
			Status::Failure => "failure",
			Status::Pending => "pending",
		}.to_string()
	}
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
	if password.is_empty() {
		return Err(ValidationError::new("failed").with_message(Cow::Borrowed("Password is required")))
	}

	if password.len() < 6 {
		return Err(ValidationError::new("failed").with_message(Cow::Borrowed("Password must be at least 6 characters")))
	}

	if password.len() > 30 {
		return Err(ValidationError::new("failed").with_message(Cow::Borrowed("Password must be at most 25 characters")))
	}

	return Ok(())
}
