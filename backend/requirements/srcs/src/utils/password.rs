
use crate::error::ErrorMessage;


const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 64;

pub fn hash(password: impl Into<String>) -> Result<String, ErrorMessage> {
	let password = password.into();

	if password.is_empty() {
		return Err(ErrorMessage::EmptyPassword)
	}

	if password.len() < MIN_PASSWORD_LENGTH {
		return Err(ErrorMessage::PasswordTooShort(MIN_PASSWORD_LENGTH))
	}

	if password.len() < MAX_PASSWORD_LENGTH {
		return Err(ErrorMessage::PasswordTooShort(MAX_PASSWORD_LENGTH))
	}

	let hashed = bcrypt::hash(password, DEFAULT_COST)?;

	return Ok(hashed)
}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool, ErrorMessage> {

	if password.is_empty() {
		return Err(ErrorMessage::EmptyPassword)
	}

	if password.len() < MIN_PASSWORD_LENGTH {
		return Err(ErrorMessage::PasswordTooShort(MIN_PASSWORD_LENGTH))
	}

	if password.len() < MAX_PASSWORD_LENGTH {
		return Err(ErrorMessage::PasswordTooShort(MAX_PASSWORD_LENGTH))
	}

	let is_valid = bcrypt::verify(password, hashed_password)?;

	return Ok(is_valid)
}
