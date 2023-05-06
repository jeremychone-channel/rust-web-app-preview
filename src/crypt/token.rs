use crate::crypt::{EncArgs, Error, Result};
use crate::utils::{
	b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_iso8601,
};
use crate::{conf, crypt};

/// The release of a parse token.
/// All properties have be b64u decoded.
pub struct Token {
	pub user: String,      // user identifier
	pub exp: String,       // expiration date in iso8601
	pub sign_b64u: String, // signature, in base64url encoded
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

/// With the format `user_ident.expiration.signature`
///
/// All these parts will be base64url endcoded:
///
/// - `user_ident` is the username in our case, but could be `user.id` or `user.uuid`
/// - `expiration` is the utc iso8601 expiration date of this token.
/// - `signature` is the token signature of the two first parts (base64url encoded) in base64url
///
pub fn generate_token(user: &str, salt: &str) -> Result<Token> {
	let duration_sec = conf().TOKEN_DURATION_SEC;

	// -- Compute the two first components.
	let user = user.to_string();
	let exp = now_utc_plus_sec_str(duration_sec);

	// -- Sign the two first components.
	let sign_b64u = sign_into_b64u(&user, &exp, salt)?;

	Ok(Token { user, exp, sign_b64u })
}

/// Validate if the origin_token signature match what it is supposed to match.
/// Returns - tuple of decoded string (user, expiration).
pub fn validate_token_sign_and_exp(origin_token: &Token, salt: &str) -> Result<()> {
	// -- Validate signature.
	let new_sign_b64u = sign_into_b64u(&origin_token.user, &origin_token.exp, salt)?;
	if new_sign_b64u != origin_token.sign_b64u {
		return Err(Error::TokenSignatureNotMatching);
	}

	// -- Validate expiration.
	let origin_exp =
		parse_iso8601(&origin_token.exp).map_err(|_| Error::TokenExpNotIso)?;
	let now = now_utc();

	if origin_exp < now {
		return Err(Error::TokenExpired);
	}

	Ok(())
}

fn sign_into_b64u(user: &str, exp: &str, salt: &str) -> Result<String> {
	let key = &conf().KEY_TOKEN;
	let content = format!("{}.{}", b64u_encode(user), b64u_encode(exp));
	let signature =
		crypt::encrypt_into_b64u(EncArgs { key, salt, content: &content })?;

	Ok(signature)
}
