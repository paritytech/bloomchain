//! In memory databse which simulates disk database reads
//! 
//! According to this article, LevelDB can perform 129.000 random read operations / s
//! http://www.lmdb.tech/bench/microbench/benchmark.html

use std::{time, thread};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use bloomchain::{Position, Bloom, BloomDatabase};
use bloomchain::group::{GroupPosition, BloomGroup, BloomGroupDatabase};

/// 1.0s / 129 000
const LEVELDB_READ_TIME: u32 = 7752;

#[derive(Default)]
pub struct BloomMemoryDatabase {
	/// Read time in nanoseconds
	read_time: u32,
	mem: HashMap<Position, Bloom>,
	queried: RefCell<HashSet<Position>>,
}

impl BloomMemoryDatabase {
	pub fn simulate_leveldb() -> Self {
		BloomMemoryDatabase {
			read_time: LEVELDB_READ_TIME,
			mem: HashMap::new(),
			queried: RefCell::default(),
		}
	}

	#[allow(dead_code)]
	pub fn insert_blooms(&mut self, blooms: HashMap<Position, Bloom>) {
		self.mem.extend(blooms);
	}
}

impl BloomDatabase for BloomMemoryDatabase {
	fn bloom_at(&self, position: &Position) -> Option<Bloom> {
		if self.read_time != 0 && !self.queried.borrow().contains(position) {
			self.queried.borrow_mut().insert(position.clone());
			thread::sleep(time::Duration::new(0, self.read_time));
		}
		self.mem.get(position).cloned()
	}
}

#[derive(Default)]
pub struct BloomGroupMemoryDatabase {
	/// Read time in nanoseconds
	read_time: u32,
	mem: HashMap<GroupPosition, BloomGroup>,
	queried: RefCell<HashSet<GroupPosition>>,
}

impl BloomGroupMemoryDatabase {
	pub fn simulate_leveldb() -> Self {
		BloomGroupMemoryDatabase {
			read_time: LEVELDB_READ_TIME,
			mem: HashMap::new(),
			queried: RefCell::default(),
		}
	}

	#[allow(dead_code)]
	pub fn insert_blooms(&mut self, groups: HashMap<GroupPosition, BloomGroup>) {
		self.mem.extend(groups);
	}
}

impl BloomGroupDatabase for BloomGroupMemoryDatabase {
	fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup> {
		if self.read_time != 0 && !self.queried.borrow().contains(position) {
			self.queried.borrow_mut().insert(position.clone());
			thread::sleep(time::Duration::new(0, self.read_time));
		}
		self.mem.get(position).cloned()
	}
}
