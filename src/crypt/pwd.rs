use super::{Error, Result};
use crate::conf;
use crate::crypt::{encrypt_into_b64u, EncryptArgs};

pub fn encrypt_pwd(args: &EncryptArgs) -> Result<String> {
	encrypt_for_scheme("", args) // "" for default scheme
}

pub fn validate_pwd(args: &EncryptArgs, pwd_ref: &str) -> Result<()> {
	let origin_scheme = get_scheme(pwd_ref)?;
	let new_pwd = encrypt_for_scheme(&origin_scheme, args)?;
	if pwd_ref == new_pwd {
		Ok(())
	} else {
		Err(Error::PwdNotMatching)
	}
}

// region:    --- Scheme Infra.
fn get_scheme(enc_content: &str) -> Result<String> {
	enc_content
		.match_indices('#')
		.nth(1)
		.map(|(index, _)| enc_content.split_at(index))
		.map(|(scheme, _)| format!("{scheme}#")) // add back the last #
		.ok_or(Error::SchemeNotFoundInContent)
}

fn encrypt_for_scheme(scheme: &str, args: &EncryptArgs) -> Result<String> {
	match scheme {
		"#01#" | "" => Ok(format!("#01#{}", Scheme01::encrypt(args)?)), // This is the default
		_ => Err(Error::SchemeUnknown(scheme.to_string())),
	}
}

trait Scheme {
	fn encrypt(enc_args: &EncryptArgs) -> Result<String>;
}
// endregion: --- Scheme Infra.

// region:    --- Scheme01
struct Scheme01;

impl Scheme for Scheme01 {
	fn encrypt(enc_pwd_args: &EncryptArgs) -> Result<String> {
		let key = &conf().KEY_PWD;

		encrypt_into_b64u(key, enc_pwd_args)
	}
}
// endregion: --- Scheme01

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
	fn test_pwd_encrypt() -> Result<()> {
		let salt = "some-salt".to_string();
		let pwd_clear = "welcome".to_string();
		let pwd_enc = encrypt_pwd(&EncryptArgs { salt, content: pwd_clear })?;

		debug!("pwd_enc: {pwd_enc}");
		Ok(())
	}

	#[test]
	fn test_pwd_validate() -> Result<()> {
		let salt = "some-salt";
		let pwd_clear = "welcome";

		let pwd_enc_1 = encrypt_pwd(&EncryptArgs {
			salt: salt.to_string(),
			content: pwd_clear.to_string(),
		})?;

		validate_pwd(
			&EncryptArgs {
				salt: salt.to_string(),
				content: pwd_clear.to_string(),
			},
			&pwd_enc_1,
		)?;

		Ok(())
	}

	#[test]
	fn test_pwd_extract_scheme() -> Result<()> {
		let s = "#01#G1Awj9k19UY2D04EQ9DCxpSIxMApGgI0Ogvg+Xi/QXoXEO1b5hAXmusXmT2wo/L8VWenfZShPT42gk7k3BZSwA==";
		assert_eq!("#01#", get_scheme(s)?);
		Ok(())
	}
}
// endregion: --- Tests
