//! Optimization gathering together blooms that are in the same index and are likely to be retrived together.

pub mod database;
pub mod position;
pub mod group;
pub mod chain;

pub use self::position::{Position, GroupPosition, Manager};
pub use self::database::{GroupDatabaseBridge, BloomGroupDatabase};
pub use self::group::BloomGroup;
