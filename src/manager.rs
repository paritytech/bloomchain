
use std::marker::PhantomData;
use std::ops::Range;
use std::path::PathBuf;
use std::fs::File;
use std::io;
use data::Data;
use number::Number;
use config;
use config::Config;
use error::Error;

/// ChainDB handle.
pub struct Manager<T> where T: Data {
	data: PhantomData<T>, 
	config: Config,
}

impl<T> Manager<T> where T: Data {
	/// Creates new manager instance with configuration saved to a file.
	/// A configuration file is created if it does not exist. If the file already exists
	/// it must contain the the configuration equal default one, otherwise the error will be returned.
	pub fn new_persistent(default: Config, path: PathBuf) -> Result<Self, Error> {
		let config = try!(config::File::load_or_create(default, &path));

		let db = Manager {
			data: PhantomData,
			config: config,
		};

		Ok(db)
	}

	/// Inserts the data at given for given key.
	pub fn insert(&self, number: Number, data: T) {
		unimplemented!();
	}

	/// Resets data in range.
	/// Inserts new data.
	/// Inserted data may exceed reseted range.
	pub fn replace(&self, range: Range<Number>, data: Vec<T>) {
		unimplemented!();
	}

	/// Returns all keys with given data.
	pub fn with_data(&self, range: Range<Number>, data: T) -> Vec<Number> {
		unimplemented!();
	}
}

#[cfg(test)]
mod tests {
	use std::{mem, env};
	use data::Data;
	use size::{Size, Bytes};
	use config::Config;
	use super::*;

	struct TestData;
	
	impl Data for TestData {
		type Raw = [u8; 32];

		fn raw(self) -> Self::Raw {
			[1; 32]
		}

		fn size() -> Size {
			Size::Bytes(Bytes(mem::size_of::<Self::Raw>()))
		}
	}

	#[test]
	fn database_open() {
		use std::fs::File;

		let mut dir = env::temp_dir();
		dir.push("chaindbx.cfg");
		let database = Manager::<TestData>::new_persistent(Config::default(), dir).unwrap();
	}

}
