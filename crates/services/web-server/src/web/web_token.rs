use crate::web::Result;
use lib_base::token::{generate_token, validate_token, Token};
use lib_core::config;

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
	let config = &config();
	let token =
		generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)?;

	Ok(token)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
	let config = &config();
	validate_token(origin_token, salt, &config.TOKEN_KEY)?;

	Ok(())
}
