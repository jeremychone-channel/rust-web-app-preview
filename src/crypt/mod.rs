pub use self::error::{Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha512;

mod error;
pub mod pwd;
pub mod token;

pub struct EncArgs<'a> {
	pub key: &'a [u8],
	pub salt: &'a str,
	pub content: &'a str,
}

pub fn encrypt_into_b64u(args: EncArgs) -> Result<String> {
	// Create a HMAC-SHA-512
	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(args.key).map_err(|_| Error::KeyFailHmac)?;

	hmac_sha512.update(args.content.as_bytes());
	hmac_sha512.update(args.salt.as_bytes());

	let hmac_result = hmac_sha512.finalize();
	let result_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&result_bytes);

	Ok(result)
}
