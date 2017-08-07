#![feature(test)]

extern crate test;
extern crate bloomchain;
extern crate tests_util as util;

use test::Bencher;
use bloomchain::{BloomChain, Config};
use util::{BloomMemoryDatabase, for_each_bloom};

#[bench]
fn file_test_bloom_search(b: &mut Bencher) {
	let config = Config::default();
	let mut db = BloomMemoryDatabase::simulate_leveldb();
	let blooms_file = include_bytes!("../tests/data/blooms.txt");

	for_each_bloom(blooms_file, | block_number, bloom | {
		let modified_blooms = {
			let chain = BloomChain::new(config, &db);
			chain.insert(block_number, bloom)
		};

		// number of modified blooms should always be equal number of levels
		assert_eq!(modified_blooms.len(), config.levels);
		db.insert_blooms(modified_blooms);
	});

	b.iter(|| {
		for_each_bloom(blooms_file, | block_number, bloom | {
			let chain = BloomChain::new(config, &db);
			let blocks = chain.with_bloom(&(block_number - 1000..block_number + 1000), &bloom);
			assert!(blocks.len() >= 1);
			assert!(blocks.contains(&block_number));
		});
	});
}
