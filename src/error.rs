
use std::io;

#[derive(Debug)]
pub enum Error {
	IoError(io::Error),
	LoadingConfigFailed,
	SavingConfigFailed,
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::IoError(error)
	}
}
