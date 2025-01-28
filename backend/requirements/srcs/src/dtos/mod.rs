pub mod user;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
	if password.is_empty() {
		return Err(ValidationError::new("failed").with_message("Password is required"))
	}

	if password.len() < 6 {
		return Err(ValidationError::new("failed").with_message("Password must be at least 6 characters"))
	}

	if password.len() > 25 {
		return Err(ValidationError::new("failed").with_message("Password must be at most 25 characters"))
	}

	return Ok(())
}
