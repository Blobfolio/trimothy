/*!
# Benchmark: Trim Slice
*/

use brunch::{
	Bench,
	benches,
};
use trimothy::{
	TrimSlice,
	TrimSliceMatches,
};
use std::time::Duration;



const BYTES: &[u8] = b"  \t\nHello World!\n\t  ";
const STR: &str = "  \t\nHello World!\n\t  ";



benches!(
	Bench::new("&[u8]", "trim()")
		.timed(Duration::from_secs(1))
		.with(|| BYTES.trim()),

	Bench::new("&str", "trim()")
		.timed(Duration::from_secs(1))
		.with(|| STR.trim()),

	Bench::spacer(),

	Bench::new("&[u8]", "trim_start()")
		.timed(Duration::from_secs(1))
		.with(|| BYTES.trim_start()),

	Bench::new("&str", "trim_start()")
		.timed(Duration::from_secs(1))
		.with(|| STR.trim_start()),

	Bench::spacer(),

	Bench::new("&[u8]", "trim_end()")
		.timed(Duration::from_secs(1))
		.with(|| BYTES.trim_end()),

	Bench::new("&str", "trim_end()")
		.timed(Duration::from_secs(1))
		.with(|| STR.trim_end()),

	Bench::spacer(),

	Bench::new("&[u8]", "trim_start_matches()")
		.timed(Duration::from_secs(1))
		.with(|| BYTES.trim_start_matches(|b| matches!(b, b'\t' | b' ' | b'\n' | b'H' | b'e'))),

	Bench::new("&str", "trim_start_matches()")
		.timed(Duration::from_secs(1))
		.with(|| STR.trim_start_matches(|c| matches!(c, '\t' | ' ' | '\n' | 'H' | 'e'))),
);
