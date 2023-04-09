use super::{Error, Result};
use crate::conf;
use crate::crypt::{encrypt_into_b64u, EncArgs};

pub struct EncPwdArgs<'a> {
	pub salt: &'a str,
	pub content: &'a str,
}

pub fn encrypt_pwd(args: EncPwdArgs) -> Result<String> {
	encrypt_for_scheme("", args) // "" for default scheme
}

pub fn validate_pwd(args: EncPwdArgs, origin_pwd: &str) -> Result<()> {
	let origin_scheme = get_scheme(origin_pwd)?;
	let new_pwd = encrypt_for_scheme(&origin_scheme, args)?;
	if origin_pwd == new_pwd {
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

fn encrypt_for_scheme(scheme: &str, args: EncPwdArgs) -> Result<String> {
	match scheme {
		"#01#" | "" => Ok(format!("#01#{}", Scheme01::encrypt(args)?)), // This is the default
		_ => Err(Error::SchemeUnknown(scheme.to_string())),
	}
}

trait Scheme {
	fn encrypt(enc_args: EncPwdArgs) -> Result<String>;
}
// endregion: --- Scheme Infra.

// region:    --- Scheme01
struct Scheme01;

impl Scheme for Scheme01 {
	fn encrypt(EncPwdArgs { content, salt }: EncPwdArgs) -> Result<String> {
		let key = &conf().KEY_PWD;

		encrypt_into_b64u(EncArgs { key, salt, content })
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

	#[test]
	fn test_pwd_encrypt() -> Result<()> {
		test_utils::init_dev_envs();

		let salt = "some-salt";
		let pwd_clear = "welcome";
		let pwd_enc = encrypt_pwd(EncPwdArgs { salt, content: pwd_clear })?;

		println!("->> pwd_enc: {pwd_enc}");
		Ok(())
	}

	#[test]
	fn test_pwd_validate() -> Result<()> {
		test_utils::init_dev_envs();
		let salt = "some-salt";
		let pwd_clear = "welcome";

		let pwd_enc_1 = encrypt_pwd(EncPwdArgs { salt, content: pwd_clear })?;

		validate_pwd(EncPwdArgs { salt, content: pwd_clear }, &pwd_enc_1)?;

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
