pub mod bridge;
pub mod memory;

pub use self::bridge::GroupDatabaseBridge;
use group::{GroupPosition, BloomGroup};

pub trait BloomGroupDatabase {
	fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup>;
}
