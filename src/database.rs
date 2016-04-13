
use std::marker::PhantomData;
use std::ops::Range;
use std::path::PathBuf;
use std::fs::File;
use std::io;
use data::Data;
use number::Number;
use config::Config;
use error::Error;

/// ChainDB handle.
// consider keeping the handle to config file, so two instances of 
pub struct Database<T> where T: Data {
	data: PhantomData<T>, 
	location: PathBuf,
}

impl<T> Database<T> where T: Data {
	/// Opens the database at given location.
	pub fn open(path: PathBuf, config: Config) -> Result<Self, Error> {
		let mut file = try!(File::open(path));
		Self::load(file, config)
	}

	/// Opens the database at given location. Creates new db, if old does not exist.
	pub fn open_or_create(path: PathBuf, config: Config) -> Result<Self, Error> {
		match File::open(&path) {
			// continue.
			Ok(file) => Self::load(file, config),
			// if it does not exist, create it.
			Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
				let mut file = try!(File::create(&path));
				try!(config.save(&mut file));
				
				let db = Database {
					data: PhantomData,
					location: path
				};

				Ok(db)
			},
			// otherwise return error.
			Err(err) => Err(From::from(err))
		}
	}

	pub fn load(mut config_file: File, config: Config) -> Result<Self, Error> {
		unimplemented!();
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
		dir.push("chaindb.cfg");
		let mut file = File::create(&dir).unwrap();
		let mut f2 = File::open(&dir).unwrap();

		//let database = Database::<TestData>::open(dir, Config::default()).unwrap();
	}

}
