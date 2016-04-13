use std::mem;
use std::io::{Read, Write};
use error::Error;

// Config size in bytes.
const CONFIG_SIZE: usize = 24;

#[derive(Debug, PartialEq)]
pub struct Config {
	/// Number of levels.
	pub levels: usize,
	/// Number of elements in a single index.
	pub elements_per_index: usize,
	/// Number of indexes in a single file.
	pub indexes_per_file: usize,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			levels: 1,
			elements_per_index: 16,
			indexes_per_file: 10_000,
		}
	}
}

impl Config {
	pub fn load(raw: &mut Read) -> Result<Config, Error> {
		assert_eq!(mem::size_of::<Config>(), CONFIG_SIZE);
		let mut buffer = [0; CONFIG_SIZE];
		let len = try!(raw.read(&mut buffer));
		match len == CONFIG_SIZE {
			true => Ok(unsafe { mem::transmute(buffer) }),
			false => Err(Error::LoadingConfigFailed)
		}
	}

	pub fn raw(&self) -> &[u8] {
		assert_eq!(mem::size_of::<Config>(), CONFIG_SIZE);
	 	unsafe { 
			mem::transmute::<&Config, &[u8; CONFIG_SIZE]>(self) 
		}
	}

	pub fn save(&self, buf: &mut Write) -> Result<(), Error> {
		let len = try!(buf.write(self.raw()));
		match len == CONFIG_SIZE {
			true => Ok(()),
			false => Err(Error::SavingConfigFailed)
		}
	}
}

#[cfg(test)]
mod tests {
	use std::mem;
	use super::Config;

	fn raw_config() {
		let config = Config::default();
		let loaded = {
			let mut raw = config.raw();
			Config::load(&mut raw).unwrap()
		};
		
		assert_eq!(config, loaded);
	}
}
