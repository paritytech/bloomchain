use std::ops::BitOr;

/// 2048 bits long hash.
pub struct Bloom([u8; 256]);

impl From<[u8; 256]> for Bloom {
	fn from(bytes: [u8; 256]) -> Self {
		Bloom(bytes)
	}
}

impl Into<[u8; 256]> for Bloom {
	fn into(self) -> [u8; 256] {
		self.0
	}
}

impl Default for Bloom {
	fn default() -> Self {
		Bloom([0u8; 256])
	}
}

impl Clone for Bloom {
	fn clone(&self) -> Self {
		let mut bloom = Bloom::default();
		bloom.0.copy_from_slice(&self.0);
		bloom
	}
}

impl BitOr for Bloom {
	type Output = Bloom;

	fn bitor(self, rhs: Self) -> Bloom {
		let mut bloom = Bloom::default();
		for i in 0..bloom.0.len() {
			bloom.0[i] = self.0[i] | rhs.0[i]
		}
		bloom
	}
}

impl <'a> BitOr for &'a Bloom {
	type Output = Bloom;

	fn bitor(self, rhs: Self) -> Bloom {
		let mut bloom = Bloom::default();
		for i in 0..bloom.0.len() {
			bloom.0[i] = self.0[i] | rhs.0[i]
		}
		bloom
	}
}

/// TODO: this functions should be moved to trait and made generic, for bloom
/// with any size.
impl Bloom {
	/// Quickly checks if this bloom contains the other bloom.
	pub fn contains(&self, bloom: &Bloom) -> bool {
		self.0.iter()
			.zip(bloom.0.iter())
			.all(|(s, b)| &(s & b) == b)
	}
}
