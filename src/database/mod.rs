pub mod memory;

use position::Position;
use bloom::Bloom;

pub trait BloomDatabase {
	fn bloom_at(&self, position: &Position) -> Option<Bloom>;
}
