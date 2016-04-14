/// Uniquely identifies bloom group position.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GroupPosition {
	/// Bloom level.
	pub level: usize,
	/// Index of the group.
	pub index: usize,
}

/// Uniquely identifies bloom position including the position in the group.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
	pub group: GroupPosition,
	/// Number in group.
	pub number: usize,
}
