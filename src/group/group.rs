use bloom::Bloom;

/// Group of blooms that were in the same index.
pub struct BloomGroup {
	pub blooms: Vec<Bloom>,
}
