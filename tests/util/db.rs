use std::collections::HashMap;
use bloomchain::{Position, Bloom, BloomDatabase};
use bloomchain::group::{GroupPosition, BloomGroup, BloomGroupDatabase};

#[derive(Default)]
pub struct BloomMemoryDatabase {
	mem: HashMap<Position, Bloom>,
}

impl BloomMemoryDatabase {
	pub fn insert(&mut self, position: Position, bloom: Bloom) {
		self.mem.insert(position, bloom);
	}
}

impl BloomDatabase for BloomMemoryDatabase {
	fn bloom_at(&self, position: &Position) -> Option<Bloom> {
		self.mem.get(position).cloned()
	}
}

#[derive(Default)]
pub struct BloomGroupMemoryDatabase {
	mem: HashMap<GroupPosition, BloomGroup>,
}

impl BloomGroupMemoryDatabase {
	pub fn insert(&mut self, position: GroupPosition, group: BloomGroup) {
		self.mem.insert(position, group);
	}
}

impl BloomGroupDatabase for BloomGroupMemoryDatabase {
	fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup> {
		self.mem.get(position).cloned()
	}
}
