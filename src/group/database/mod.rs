pub mod bridge;

use group::{GroupPosition, BloomGroup};

pub trait BloomGroupDatabase {
	fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup>;
}
