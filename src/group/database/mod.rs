mod bridge;
pub mod memory;

pub use self::bridge::GroupDatabaseBridge;
use group::{GroupPosition, BloomGroup};

/// Readonly `BloomGroup` database.
pub trait BloomGroupDatabase {
	fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup>;
}
