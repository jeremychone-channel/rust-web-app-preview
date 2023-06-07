use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent};
use crate::crypt::{Error, Result};
use crate::utils::{
	b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc,
};
use std::str::FromStr;

// region:    --- Token Type

/// Token with the string serialized format as
/// `ident_b64u.exp_b64u.sign_b64u`
pub struct Token {
	pub ident: String,     // identifier (username for example).
	pub exp: String,       // Expiration date in Rfc3339.
	pub sign_b64u: String, // Signature, base64url encoded.
}

impl FromStr for Token {
	type Err = Error;

	fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
		let splits: Vec<&str> = token_str.split('.').collect();
		if splits.len() != 3 {
			return Err(Error::TokenInvalidFormat);
		}
		let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);
		Ok(Self {
			ident: b64u_decode(ident_b64u)
				.map_err(|_| Error::TokenCannotDecodeUser)?,
			exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
			sign_b64u: sign_b64u.to_string(),
		})
	}
}

impl std::fmt::Display for Token {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter,
	) -> core::result::Result<(), std::fmt::Error> {
		write!(
			fmt,
			"{}.{}.{}",
			b64u_encode(&self.ident),
			b64u_encode(&self.exp),
			self.sign_b64u
		)
	}
}

// endregion: --- Token Type

// region:    --- Web Token Gen and Validation

/// Generate a Token for a given user identifier and its token salt.
pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
	let config::Config { TOKEN_DURATION_SEC, TOKEN_KEY, .. } = &config();
	_generate_token(user, salt, *TOKEN_DURATION_SEC, TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
	let config::Config { TOKEN_KEY, .. } = &config();
	_validate_token_sign_and_exp(origin_token, salt, TOKEN_KEY)?;
	Ok(())
}

// endregion: --- Web Token Validation

// region:    --- (private) Token Gen and Validation

fn _generate_token(
	ident: &str,
	salt: &str,
	duration_sec: f64,
	key: &[u8],
) -> Result<Token> {
	// -- Compute the two first components.
	let ident = ident.to_string();
	let exp = now_utc_plus_sec_str(duration_sec);

	// -- Sign the two first components.
	let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

	Ok(Token { ident, exp, sign_b64u })
}

/// Validate if the origin_token signature match what it is supposed to match.
/// Returns - tuple of decoded string (ident, expiration).
fn _validate_token_sign_and_exp(
	origin_token: &Token,
	salt: &str,
	key: &[u8],
) -> Result<()> {
	// -- Validate signature.
	let new_sign_b64u =
		_token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

	if new_sign_b64u != origin_token.sign_b64u {
		return Err(Error::TokenSignatureNotMatching);
	}

	// -- Validate expiration.
	let origin_exp =
		parse_utc(&origin_token.exp).map_err(|_| Error::TokenExpNotIso)?;
	let now = now_utc();

	if origin_exp < now {
		return Err(Error::TokenExpired);
	}

	Ok(())
}

/// Create a token signature given the ident identifier,
/// expiration, and salt.
fn _token_sign_into_b64u(
	ident: &str,
	exp: &str,
	salt: &str,
	key: &[u8],
) -> Result<String> {
	let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
	let signature =
		encrypt_into_b64u(key, &EncryptContent { content, salt: salt.to_string() })?;

	Ok(signature)
}

// endregion: --- Private Token Gen and Validation

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use std::thread;
	use std::time::Duration;

	#[test]
	fn test_validate_web_token_ok() -> Result<()> {
		// -- Setup & Fixtures
		let fx_user = "user_one";
		let fx_salt = "pepper";
		let fx_duration_sec = 0.02; // 10ms
		let token_key = &config().TOKEN_KEY;
		let fx_token =
			_generate_token(fx_user, fx_salt, fx_duration_sec, token_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(10));
		let res = validate_web_token(&fx_token, fx_salt);

		// -- Check
		res?;

		Ok(())
	}

	#[test]
	fn test_validate_web_token_err() -> Result<()> {
		// -- Setup & Fixtures
		let fx_user = "user_one";
		let fx_salt = "pepper";
		let fx_duration_sec = 0.01; // 10ms
		let token_key = &config().TOKEN_KEY;
		let fx_token =
			_generate_token(fx_user, fx_salt, fx_duration_sec, token_key)?;

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
