extern crate bloomchain;

mod util;

use util::db::BloomMemoryDatabase;

#[test]
fn it_works() {
	let memory_db = BloomMemoryDatabase::default();
}
