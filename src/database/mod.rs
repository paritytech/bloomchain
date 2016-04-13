use position::Position;
use bloom::Bloom;

pub mod memory;

pub trait BloomDatabase {
	fn bloom_at(&self, position: &Position) -> Option<Bloom>;
}
