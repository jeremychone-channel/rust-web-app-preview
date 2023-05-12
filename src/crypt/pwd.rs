use super::{Error, Result};
use crate::conf;
use crate::crypt::{encrypt_into_b64u, EncryptContent};
use lazy_regex::regex_captures;

pub const DEFAULT_SCHEME: &str = "02";

/// Encrypt the password with the default scheme.
pub fn encrypt_pwd(args: &EncryptContent) -> Result<String> {
	encrypt_for_scheme(DEFAULT_SCHEME, args) // "" for default scheme
}

pub enum SchemeStatus {
	Ok,       // The pwd use the latest scheme. All good.
	Outdated, // The pwd use a old scheme. Would need to be re-encrypted.
}
/// Validation if an EncryptContent matches
/// the encrypted pwd reference (usually coming from `user.pwd`)
pub fn validate_pwd(args: &EncryptContent, pwd_ref: &str) -> Result<SchemeStatus> {
	let origin_scheme = extract_scheme(pwd_ref)?;
	let new_pwd = encrypt_for_scheme(&origin_scheme, args)?;
	if pwd_ref == new_pwd {
		if origin_scheme == DEFAULT_SCHEME {
			Ok(SchemeStatus::Ok)
		} else {
			Ok(SchemeStatus::Outdated)
		}
	} else {
		Err(Error::PwdNotMatching)
	}
}

// region:    --- Scheme Infra
fn encrypt_for_scheme(scheme: &str, args: &EncryptContent) -> Result<String> {
	let pwd = match scheme {
		"01" => encrypt_scheme_01(args),
		DEFAULT_SCHEME => encrypt_scheme_02(args),
		_ => Err(Error::SchemeUnknown(scheme.to_string())),
	};

	Ok(format!("#{scheme}#{}", pwd?))
}

fn extract_scheme(enc_content: &str) -> Result<String> {
	regex_captures!(
		r#"^#(\w+)#.*"#, // a literal regex
		enc_content
	)
	.map(|(_whole, scheme)| scheme.to_string())
	.ok_or(Error::SchemeNotFoundInContent)
}

fn encrypt_scheme_01(enc_content: &EncryptContent) -> Result<String> {
	let key = &conf().PWD_KEY;

	encrypt_into_b64u(key, enc_content)
}

// In this example, same a scheme_01 (showing that it works)
fn encrypt_scheme_02(enc_pwd_args: &EncryptContent) -> Result<String> {
	let key = &conf().PWD_KEY;

	encrypt_into_b64u(key, enc_pwd_args)
}
// endregion: --- Scheme Infra

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use super::*;
	use crate::test_utils;
	use anyhow::Result;
	use rand::RngCore;
	use tracing::debug;

	#[test]
	fn test_encrypt() -> Result<()> {
		let salt = "some-salt".to_string();
		let pwd_clear = "welcome".to_string();
		let pwd_enc = encrypt_pwd(&EncryptContent { salt, content: pwd_clear })?;

		debug!("pwd_enc: {pwd_enc}");
		Ok(())
	}

	#[test]
	fn test_validate() -> Result<()> {
		let salt = "some-salt";
		let pwd_clear = "welcome";

		let pwd_enc_1 = encrypt_pwd(&EncryptContent {
			salt: salt.to_string(),
			content: pwd_clear.to_string(),
		})?;

		validate_pwd(
			&EncryptContent {
				salt: salt.to_string(),
				content: pwd_clear.to_string(),
			},
			&pwd_enc_1,
		)?;

		Ok(())
	}

	#[test]
	fn test_extract_scheme() -> Result<()> {
		let s = "#01#G1Awj9k19UY2D04EQ9DCxpSIxMApGgI0Ogvg+Xi/QXoXEO1b5hAXmusXmT2wo/L8VWenfZShPT42gk7k3BZSwA==";
		assert_eq!("01", extract_scheme(s)?);
		Ok(())
	}
}
// endregion: --- Tests
