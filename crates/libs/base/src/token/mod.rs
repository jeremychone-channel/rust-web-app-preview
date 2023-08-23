//! Design:
//!
//! - A `Token` serves as a reusable construct designed to sign a specific identifier with an associated expiration date.
//! - The string format of a token follows the pattern: `identifier_b64u.expiration_rfc3339_b64u.signature_b64u`.
//! - Each segment is encoded using base64 URL encoding to ensure maximum portability.
//! - Tokens can be used for various purposes such as Web tokens, password reset tokens, signed URLs, or any other application that requires a string identifier with an expiration.
//! - This Token implementation is utilized in `services/web-server/src/web/web_token.rs`.
//! - Currently, the token employs a singular encryption scheme, which is typically sufficient. However, if necessary, it can be expanded to support multiple schemes (refer to `core/pwd/` for an example of a multi-scheme pattern).

// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use crate::b64::{b64u_decode_into_string, b64u_encode, b64u_encode_bytes};
use crate::time::{now_utc, now_utc_plus_sec_str, parse_utc};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use std::fmt::Display;
use std::str::FromStr;

// endregion: --- Modules

// region:    --- Token

/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
pub struct Token {
	pub ident: String,     // Identifier (username for example).
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
			ident: b64u_decode_into_string(ident_b64u)
				.map_err(|_| Error::TokenCannotDecodeIdent)?,

			exp: b64u_decode_into_string(exp_b64u)
				.map_err(|_| Error::TokenCannotDecodeExp)?,

			sign_b64u: sign_b64u.to_string(),
		})
	}
}

impl Display for Token {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter,
	) -> core::result::Result<(), std::fmt::Error> {
		write!(
			f,
			"{}.{}.{}",
			b64u_encode(&self.ident),
			b64u_encode(&self.exp),
			self.sign_b64u
		)
	}
}

// endregion: --- Token

// region:    --- Generate, Sign, Validate

/// Generate a new token given the identifier, duration_sec (for computing new expiration),
/// and salt and key.
///
/// See `sign_token()` for more info on the signature.
pub fn generate_token(
	ident: &str,
	duration_sec: f64,
	salt: &str,
	key: &[u8],
) -> Result<Token> {
	// -- Compute the two first components.
	let ident = ident.to_string();
	let exp = now_utc_plus_sec_str(duration_sec);

	// -- Sign the two first components.
	let sign_b64u = sign_token_into_b64u(&ident, &exp, salt, key)?;

	Ok(Token { ident, exp, sign_b64u })
}

/// Validate if the origin_token signature match and the expiration.
pub fn validate_token(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
	// -- Validate signature.
	let new_sign_b64u =
		sign_token_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

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

/// Sign a token content (identifier & expiration) given a salt and a key.
///
/// Note 1: For the token, the current design aims to support the same signature for all token types.
///         If there's a need to support multiple signature types, a "signator" trait pattern can be utilized.
/// Note 2: At the moment, HMAC/SHA512 is the choice for the signature. However, an alternative approach
///         could be to use SHA512 with the key as a salt. This method could offer potential performance
///         advantages and still be valid for token signature purposes.
pub fn sign_token_into_b64u(
	ident: &str,
	exp: &str,
	salt: &str,
	key: &[u8],
) -> Result<String> {
	let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

	// -- Create a HMAC-SHA-512 from key.
	let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key)
		.map_err(|_| Error::HmacFailNewFromSlice)?;

	// -- Add content.
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	// -- Finalize and b64u encode.
	let hmac_result = hmac_sha512.finalize();
	let result_bytes = hmac_result.into_bytes();
	let result = b64u_encode_bytes(&result_bytes);

	Ok(result)
}

// endregion: --- Generate, Sign, Validate

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::b64::b64u_decode;
	use anyhow::Result;
	use std::thread;
	use std::time::Duration;

	const TEST_KEY: &str = "3lXyiZQyDRJRiGDA5u5oN8fXCfKkymqQNWwQ_0VbmPoSp7c6kHcfizI5LhTdcl5-zWwgdUEbHaed5h__TBI5ug";
	fn get_fx_key() -> Vec<u8> {
		b64u_decode(TEST_KEY).unwrap()
	}

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
	fn test_validate_exp_ok() -> Result<()> {
		// -- Setup & Fixtures
		let fx_user = "user_one";
		let fx_salt = "pepper";
		let fx_duration_sec = 0.02; // 20ms
		let fx_key = get_fx_key();
		let fx_token = generate_token(fx_user, fx_duration_sec, fx_salt, &fx_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(10));
		let res = validate_token(&fx_token, fx_salt, &fx_key);

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
		let fx_key = get_fx_key();
		let fx_token = generate_token(fx_user, fx_duration_sec, fx_salt, &fx_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(20));
		let res = validate_token(&fx_token, fx_salt, &fx_key);

		// -- Check
		assert!(
			matches!(res, Err(Error::TokenExpired)),
			"Should have matched `Err(Error::TokenExpired)` but was `{res:?}`"
		);

		Ok(())
	}
}
// endregion: --- Tests
