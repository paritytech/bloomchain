mod config;
mod error;
mod file;

pub use self::config::Config;
pub use self::error::Error;
pub use self::file::File;
use std::path::PathBuf;

/// Validates configuration.
/// A configuration file is created if it does not exist. If the file already exists
/// it must contain the the configuration equal default one, otherwise the error will be returned.
pub fn validate_config(default: Config, path: PathBuf) -> Result<(), Error> {
	let _ = try!(File::load_or_create(default, &path));
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::env;
	use super::*;

	#[test]
	fn config_validation() {
		let mut dir = env::temp_dir();
		dir.push("chaindbxw.cfg");
		validate_config(Config::default(), dir).unwrap();
	}
}

