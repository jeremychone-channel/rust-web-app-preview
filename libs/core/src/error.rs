use crate::crypt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Config
	ConfigMissingEnv(&'static str),
	ConfigWrongFormat(&'static str),

	// -- Modules
	Crypt(crypt::Error),
}
// region:    --- Froms
impl From<crypt::Error> for Error {
	fn from(val: crypt::Error) -> Self {
		Error::Crypt(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter,
	) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
