use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Success,
    Failure,
    Error,
    Pending,
}

impl From<Status> for String {
    fn from(value: Status) -> Self {
        value.to_string()
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Success => write!(f, "success"),
            Status::Error => write!(f, "error"),
            Status::Failure => write!(f, "failure"),
            Status::Pending => write!(f, "pending"),
        }
    }
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.is_empty() {
        return Err(
            ValidationError::new("failed").with_message(Cow::Borrowed("Password is required"))
        );
    }

    if password.len() < 6 {
        return Err(ValidationError::new("failed")
            .with_message(Cow::Borrowed("Password must be at least 6 characters")));
    }

    if password.len() > 30 {
        return Err(ValidationError::new("failed")
            .with_message(Cow::Borrowed("Password must be at most 25 characters")));
    }

    Ok(())
}
