// use argon2::{password_hash::rand_core::{OsRng, RngCore}, Argon2, Params, PasswordHash};
// use dotenv::dotenv;
// use super::constant;



// fn hash_password(password: String) -> Result<String, argon2::Error> {
// 	let salt = [0; 16];
// 	OsRng.fill_bytes(&mut salt);

// 	let params = Params::new(
// 		65536,			// memory cost
// 		3,				// iterations cost
// 		1,				// paralellism
// 		Some(32)	// hash size
// 	)?;

// 	let argon2 = Argon2::new(
// 		argon2::Algorithm::Argon2id,
// 		argon2::Version::V0x13,
// 		params,
// 	);

// 	let hash = argon2.hash_password(
// 			password.as_bytes(),
// 			&salt,
// 		)?
// 		.to_string();

// 	Ok(hash)
// }