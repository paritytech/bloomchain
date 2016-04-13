use std::io::Read;
use std::ops::BitOr;
use size::Size;

pub trait Data {
	type Raw; //: Read + BitOr;

	fn raw(self) -> Self::Raw;
	fn size() -> Size;
}
