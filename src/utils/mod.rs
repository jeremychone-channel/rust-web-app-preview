// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

// endregion: --- Modules

// region:    --- Time
pub fn now_utc() -> OffsetDateTime {
	OffsetDateTime::now_utc()
}

// TODO: Need to validate if the .unwrap below are 100% safe.
pub fn format_time(time: OffsetDateTime) -> String {
	// Because native time format, safe enough to unwrap.
	time.format(&Rfc3339).unwrap()
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
	let new_time = now_utc() + Duration::seconds_f64(sec);

	format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
	OffsetDateTime::parse(moment, &Rfc3339)
		.map_err(|_| Error::DateFailParse(moment.to_string()))
}
// endregion: --- Time

// region:    --- Base64
pub fn b64u_encode(content: &str) -> String {
	base64_url::encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<String> {
	let user_ident = base64_url::decode(b64u)
		.ok()
		.and_then(|r| String::from_utf8(r).ok())
		.ok_or(Error::FailToB64uDecode)?;

	Ok(user_ident)
}
// endregion: --- Base64
