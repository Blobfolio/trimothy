/*!
# Benchmark: Trim and Normalize
*/

use brunch::{
	Bench,
	benches,
};
use trimothy::{
	TrimNormal,
	TrimNormalBytes,
	TrimNormalChars,
};



const BYTES: &[u8] = b" H\r\nE\tL    L\tO  ";
const STR: &str = " H\r\nE\u{2001}L  \u{3000}\u{205f}L\tO  ";



benches!(
	Bench::new("&[u8]::trim_and_normalize()")
		.run(|| BYTES.trim_and_normalize()),

	Bench::new("Iterator::<Item=u8>::trim_and_normalize()")
		.run(|| BYTES.iter().copied().trim_and_normalize().collect::<Vec<_>>()),

	Bench::spacer(),

	Bench::new("&str::trim_and_normalize()")
		.run(|| STR.trim_and_normalize()),

	Bench::new("Iterator::<Item=char>::trim_and_normalize()")
		.run(|| STR.chars().trim_and_normalize().collect::<String>()),
);
