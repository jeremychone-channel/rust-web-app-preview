pub use self::error::{Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha512;

mod error;
pub mod pwd;
pub mod token;

pub fn encrypt_into_b64u(key: &[u8], enc_args: &EncryptArgs) -> Result<String> {
	let EncryptArgs { content, salt } = enc_args;

	// Create a HMAC-SHA-512
	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	let hmac_result = hmac_sha512.finalize();
	let result_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&result_bytes);

	Ok(result)
}

/// The "content" parts of an encrypt command.
/// For example, the `salt` might be per user.
/// However, `key` can be context (i.e., different for pwd-scheme, token, or reset password)
pub struct EncryptArgs {
	pub content: String, // clear content
	pub salt: String,
}
