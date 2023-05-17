use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent};
use crate::crypt::{Error, Result};
use crate::utils::{
	b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc,
};

/// Token with the string serialized format as
/// `user_b64u.exp_b64u.sign_b64u`
pub struct Token {
	pub user: String,      // User identifier (username for now).
	pub exp: String,       // Expiration date in Rfc3339.
	pub sign_b64u: String, // Signature, in base64url encoded.
}

impl Token {
	pub fn parse(token: &str) -> Result<Token> {
		let splits: Vec<&str> = token.split('.').collect();
		if splits.len() != 3 {
			return Err(Error::TokenInvalidFormat);
		}
		let (user_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);
		Ok(Self {
			user: b64u_decode(user_b64u)
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
			b64u_encode(&self.user),
			b64u_encode(&self.exp),
			self.sign_b64u
		)
	}
}

/// Generate a Token for a given user identifier and its token salt.
pub fn generate_token(user: &str, salt: &str) -> Result<Token> {
	let duration_sec = config().TOKEN_DURATION_SEC;

	// -- Compute the two first components.
	let user = user.to_string();
	let exp = now_utc_plus_sec_str(duration_sec);

	// -- Sign the two first components.
	let sign_b64u = token_sign_into_b64u(&user, &exp, salt)?;

	Ok(Token { user, exp, sign_b64u })
}

/// Validate if the origin_token signature match what it is supposed to match.
/// Returns - tuple of decoded string (user, expiration).
pub fn validate_token_sign_and_exp(origin_token: &Token, salt: &str) -> Result<()> {
	// -- Validate signature.
	let new_sign_b64u =
		token_sign_into_b64u(&origin_token.user, &origin_token.exp, salt)?;

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

/// Create a token signature given the user identifier,
/// expiration, and salt.
fn token_sign_into_b64u(user: &str, exp: &str, salt: &str) -> Result<String> {
	let key = &config().TOKEN_KEY;
	let content = format!("{}.{}", b64u_encode(user), b64u_encode(exp));
	let signature =
		encrypt_into_b64u(key, &EncryptContent { content, salt: salt.to_string() })?;

	Ok(signature)
}
