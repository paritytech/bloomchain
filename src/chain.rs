use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Range;
use number::Number;
use position::Position;
use error::Error;
use bloom::Bloom;
use filter::Filter;
use config::Config;
use database::BloomDatabase;

/// Manages all bloom database transactions.
pub struct BloomChain<'a> {
	config: Config,
	db: &'a BloomDatabase,
}

impl<'a> BloomChain<'a> {
	/// Creates new bloom chain.
	pub fn new(config: Config, db: &'a BloomDatabase) -> Self {

		BloomChain {
			config: config,
			db: db,
		}
	}

	/// Inserts the data at given for given key.
	pub fn insert(&self, number: Number, data: Bloom) -> HashMap<Position, Bloom> {
		unimplemented!();
	}

	/// Resets data in range.
	/// Inserts new data.
	/// Inserted data may exceed reseted range.
	pub fn replace(&self, range: Range<Number>, data: Vec<Bloom>) -> HashMap<Position, Bloom> {
		unimplemented!();
	}

	/// Returns all numbers with given bloom.
	pub fn with_bloom(&self, range: Range<Number>, bloom: Bloom) -> Vec<Number> {
		unimplemented!();
	}

	/// Filter the chain with given filter.
	pub fn filter(&self, filter: &Filter) -> Vec<Number> {
		let range = filter.range();
		let mut blocks = filter.bloom_possibilities()
			.into_iter()
			.flat_map(|bloom| self.with_bloom(range.clone(), bloom))
			.collect::<HashSet<Number>>()
			.into_iter()
			.collect::<Vec<Number>>();

		blocks.sort();
		blocks
	}
}

#[cfg(test)]
mod tests {
	use config::Config;
	use database::memory::Memory;
	use super::*;

	#[test]
	fn manager_with_noop_bridge() {
		let memory = Memory::default();
		let manager = BloomChain::new(Config::default(), &memory);
	}
}

