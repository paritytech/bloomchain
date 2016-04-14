use std::collections::HashMap;
use group::{GroupPosition, BloomGroup, BloomGroupDatabase};

#[derive(Default)]
pub struct Memory {
	mem: HashMap<GroupPosition, BloomGroup>,
}

impl Memory {
	pub fn insert(&mut self, position: GroupPosition, group: BloomGroup) {
		self.mem.insert(position, group);
	}
}

impl BloomGroupDatabase for Memory {
	fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup> {
		self.mem.get(position).cloned()
	}
}
