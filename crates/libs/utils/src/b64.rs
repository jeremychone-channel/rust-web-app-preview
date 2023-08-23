use base64::{engine::general_purpose, Engine as _};

pub fn b64u_encode(content: &str) -> String {
	b64u_encode_bytes(content.as_bytes())
}

pub fn b64u_encode_bytes(bytes: &[u8]) -> String {
	general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

pub fn b64u_decode_into_string(b64u: &str) -> Result<String> {
	let decoded_string = general_purpose::URL_SAFE_NO_PAD
		.decode(b64u)
		.ok()
		.and_then(|r| String::from_utf8(r).ok())
		.ok_or(Error::FailToB64uDecode)?;

	Ok(decoded_string)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
	general_purpose::URL_SAFE_NO_PAD
		.decode(b64u)
		.map_err(|_| Error::FailToB64uDecode)
}

// region:    --- Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	FailToB64uDecode,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion: --- Error
