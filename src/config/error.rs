use std::io;

#[derive(Debug)]
pub enum Error {
	IoError(io::Error),
	Loading,
	Saving,
	Mismatch,
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::IoError(error)
	}
}
