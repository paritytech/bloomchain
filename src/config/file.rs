use std::{fs, io};
use std::path::PathBuf;
use super::{Config, Error};

pub struct File;

impl File {
	pub fn load_or_create(default: Config, path: &PathBuf) -> Result<Config, Error> {
		match fs::File::open(path) {
			// config exists
			Ok(ref mut file) => {
				let config = try!(Config::load(file));
				match config == default {
					true => Ok(config),
					false => Err(Error::Mismatch)
				}
			},
			// needs to be created
			Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
				let mut file = try!(fs::File::create(path));
				try!(default.save(&mut file));
				Ok(default)
			},
			// otherwise return error.
			Err(err) => Err(From::from(err))
		}
	}
}
