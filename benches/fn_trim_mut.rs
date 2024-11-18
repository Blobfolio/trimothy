/*!
# Benchmark: Trim Mut
*/

use brunch::{
	Bench,
	benches,
};
use trimothy::TrimMut;



const BYTES: &[u8] = b"  \t\nHello World!\n\t  ";
const STR: &str = "  \t\nHello World!\n\t  ";



benches!(
	Bench::new("Vec<u8>::trim_mut()")
		.run_seeded(BYTES.to_vec(), |mut v| v.trim_mut()),

	Bench::spacer(),

	Bench::new("String::trim_mut()")
		.run_seeded(STR.to_owned(), |mut s| s.trim_mut()),

	Bench::new("String.trim()::to_owned()")
		.run(|| STR.trim().to_owned()),
);
