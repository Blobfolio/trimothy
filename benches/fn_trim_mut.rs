/*!
# Benchmark: Trim Slice
*/

use brunch::{
	Bench,
	benches,
};
use trimothy::TrimMut;
use std::time::Duration;



const BYTES: &[u8] = b"  \t\nHello World!\n\t  ";
const STR: &str = "  \t\nHello World!\n\t  ";



benches!(
	Bench::new("Vec<u8>", "trim_mut()")
		.timed(Duration::from_secs(1))
		.with_setup(BYTES.to_vec(), |mut v| v.trim_mut()),

	Bench::spacer(),

	Bench::new("String", "trim_mut()")
		.timed(Duration::from_secs(1))
		.with_setup(STR.to_owned(), |mut s| s.trim_mut()),

	Bench::new("String.trim()", "to_owned()")
		.timed(Duration::from_secs(1))
		.with_setup(STR.to_owned(), |s| s.trim().to_owned()),
);
