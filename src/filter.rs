use std::ops::Range;
use bloom::Bloom;
use number::Number;

/// Chain filter.
pub trait Filter {
	/// All bloom possibilities that we are searching for.
	fn bloom_possibilities(&self) -> Vec<Bloom>;
	/// Range of search.
	fn range(&self) -> Range<Number>;
}
