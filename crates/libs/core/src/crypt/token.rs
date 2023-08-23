use crate::crypt::Result;
use lib_utils::config;
use lib_utils::token::{generate_token, validate_token, Token};

// region:    --- Web Token Gen and Validation

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
	let config = &config();
	generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
	let config = &config();
	validate_token(origin_token, salt, &config.TOKEN_KEY)?;

	Ok(())
}

// endregion: --- Web Token Gen and Validation

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use std::thread;
	use std::time::Duration;

	#[test]
	fn test_token_display_ok() -> Result<()> {
		// -- Fixtures
		let fx_token_str =
			"ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
		let fx_token = Token {
			ident: "fx-ident-01".to_string(),
			exp: "2023-05-17T15:30:00Z".to_string(),
			sign_b64u: "some-sign-b64u-encoded".to_string(),
		};

		// -- Exec & Check
		assert_eq!(fx_token.to_string(), fx_token_str);

		Ok(())
	}

	#[test]
	fn test_token_from_str_ok() -> Result<()> {
		// -- Fixtures
		let fx_token_str =
			"ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
		let fx_token = Token {
			ident: "fx-ident-01".to_string(),
			exp: "2023-05-17T15:30:00Z".to_string(),
			sign_b64u: "some-sign-b64u-encoded".to_string(),
		};

		// -- Exec
		let token: Token = fx_token_str.parse()?;

		// -- Check
		assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));

		Ok(())
	}

	#[test]
	fn test_validate_web_token_ok() -> Result<()> {
		// -- Setup & Fixtures
		let fx_user = "user_one";
		let fx_salt = "pepper";
		let fx_duration_sec = 0.02; // 20ms
		let token_key = &config().TOKEN_KEY;
		let fx_token =
			_generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(10));
		let res = validate_web_token(&fx_token, fx_salt);

		// -- Check
		res?;

		Ok(())
	}

	#[test]
	fn test_validate_web_token_err_expired() -> Result<()> {
		// -- Setup & Fixtures
		let fx_user = "user_one";
		let fx_salt = "pepper";
		let fx_duration_sec = 0.01; // 10ms
		let token_key = &config().TOKEN_KEY;
		let fx_token =
			_generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(20));
		let res = validate_web_token(&fx_token, fx_salt);

		// -- Check
		assert!(
			matches!(res, Err(Error::TokenExpired)),
			"Should have matched `Err(Error::TokenExpired)` but was `{res:?}`"
		);

		Ok(())
	}
}
// endregion: --- Tests
