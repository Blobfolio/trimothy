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



const BYTES: &[u8] = b"  \t\nHello World!\n\t  ";
const STR: &str = "  \t\nHello World!\n\t  ";



benches!(
	Bench::new("&[u8]::trim()")
		.run(|| BYTES.trim()),

	Bench::new("&str::trim()")
		.run(|| STR.trim()),

	Bench::spacer(),

	Bench::new("&[u8]::trim_start()")
		.run(|| BYTES.trim_start()),

	Bench::new("&str::trim_start()")
		.run(|| STR.trim_start()),

	Bench::spacer(),

	Bench::new("&[u8]::trim_end()")
		.run(|| BYTES.trim_end()),

	Bench::new("&str::trim_end()")
		.run(|| STR.trim_end()),

	Bench::spacer(),

	Bench::new("&[u8]::trim_start_matches()")
		.run(|| BYTES.trim_start_matches(|b| matches!(b, b'\t' | b' ' | b'\n' | b'H' | b'e'))),

	Bench::new("&str::trim_start_matches()")
		.run(|| STR.trim_start_matches(|c| matches!(c, '\t' | ' ' | '\n' | 'H' | 'e'))),
);
