use super::{Error, Result};
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent};
use lazy_regex::regex_captures;

pub const DEFAULT_SCHEME: &str = "02";

/// Encrypt the password with the default scheme.
pub fn encrypt_pwd(enc_content: &EncryptContent) -> Result<String> {
	encrypt_for_scheme(DEFAULT_SCHEME, enc_content)
}

pub enum SchemeStatus {
	Ok,       // The pwd use the latest scheme. All good.
	Outdated, // The pwd use a old scheme. Would need to be re-encrypted.
}
/// Validate if an EncryptContent matches
pub fn validate_pwd(
	enc_content: &EncryptContent,
	pwd_ref: &str,
) -> Result<SchemeStatus> {
	let origin_scheme = extract_scheme(pwd_ref)?;
	let new_pwd = encrypt_for_scheme(&origin_scheme, enc_content)?;

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
	let key = &config().PWD_KEY;

	encrypt_into_b64u(key, enc_content)
}

// In this example, same a scheme_01 (showing that it works)
fn encrypt_scheme_02(enc_pwd_args: &EncryptContent) -> Result<String> {
	let key = &config().PWD_KEY;

	encrypt_into_b64u(key, enc_pwd_args)
}
// endregion: --- Scheme Infra

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use super::*;
	use anyhow::Result;

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
		let s = "#01#DdVzPPKKpjs-xuf-Y88t3MpQ5KPDqa7C2gpaTIysHnHIzX_j2IgNb3WtEDHLfF2ps1OWVPKOkgLFvvDMvNrN-A";
		assert_eq!(extract_scheme(s)?, "01");
		Ok(())
	}
}
// endregion: --- Tests
