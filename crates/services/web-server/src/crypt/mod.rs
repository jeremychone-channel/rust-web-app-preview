use lib_core::config;
use lib_utils::token::{self, generate_token, validate_token, Token};

pub fn generate_web_token(user: &str, salt: &str) -> token::Result<Token> {
	let config = &config();
	generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> token::Result<()> {
	let config = &config();
	validate_token(origin_token, salt, &config.TOKEN_KEY)
}
