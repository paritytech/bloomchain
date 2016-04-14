use std::collections::HashMap;
use position::Position;
use bloom::Bloom;
use super::BloomDatabase;

#[derive(Default)]
pub struct Memory {
	mem: HashMap<Position, Bloom>,
}

impl Memory {
	pub fn insert(&mut self, position: Position, bloom: Bloom) {
		self.mem.insert(position, bloom);
	}
}

impl BloomDatabase for Memory {
	fn bloom_at(&self, position: &Position) -> Option<Bloom> {
		self.mem.get(position).cloned()
	}
}
