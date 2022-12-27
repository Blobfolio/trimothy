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
		.run(|| BYTES.trim_start_matches(|b| b.is_ascii_whitespace() || matches!(b, b'H' | b'e'))),

	Bench::new("&str::trim_start_matches()")
		.run(|| STR.trim_start_matches(|c: char| c.is_ascii_whitespace() || matches!(c, 'H' | 'e'))),

	Bench::spacer(),

	Bench::new("&[u8]::trim_end_matches()")
		.run(|| BYTES.trim_end_matches(|b| b.is_ascii_whitespace() || matches!(b, b'd' | b'!'))),

	Bench::new("&str::trim_end_matches()")
		.run(|| STR.trim_end_matches(|c: char| c.is_ascii_whitespace() || matches!(c, 'd' | '!'))),
);
