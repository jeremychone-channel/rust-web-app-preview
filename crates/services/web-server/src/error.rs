use lib_core::{model, pwd};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Config
	ConfigMissingEnv(&'static str),
	ConfigWrongFormat(&'static str),

	// -- Modules
	Crypt(pwd::Error),
	Model(model::Error),
}

// region:    --- Froms
impl From<pwd::Error> for Error {
	fn from(val: pwd::Error) -> Self {
		Error::Crypt(val)
	}
}

impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Error::Model(val)
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
