/*!
# Trimothy - Trim Slice
*/

use alloc::{
	boxed::Box,
	vec::Vec,
};
use crate::pattern::MatchPattern;



/// # Trim Slice (Matches).
///
/// The [`TrimSliceMatches`] trait brings arbitrary match-based trimming support
/// to `&[u8]`, `Vec<u8>`, and `Box<[u8]>` types, very similar to the ones
/// enjoyed by `String`/`&str`.
///
/// The trait methods included are:
///
/// | Method | Description |
/// | ------ | ----------- |
/// | `trim_matches` | Trim arbitrary leading and trailing bytes. |
/// | `trim_start_matches` | Trim arbitrary leading bytes. |
/// | `trim_end_matches` | Trim arbitrary trailing bytes. |
///
/// Each of these match methods accept either:
/// * A single `u8`;
/// * An array or slice of `u8`;
/// * A `&BTreeSet<u8>`;
/// * A callback with the signature `Fn(u8) -> bool`;
pub trait TrimSliceMatches {
	/// # Trim Matches.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
	///
	/// ```
	/// use trimothy::TrimSliceMatches;
	///
	/// let s: &[u8] = b"...Custom Trim!...";
	/// assert_eq!(s.trim_matches(b'.'), b"Custom Trim!");
	/// assert_eq!(s.trim_matches([b'.']), b"Custom Trim!");
	/// assert_eq!(s.trim_matches(&[b'.']), b"Custom Trim!");
	/// assert_eq!(s.trim_matches(|b| b'.' == b), b"Custom Trim!");
	/// ```
	fn trim_matches<P: MatchPattern<u8>>(&self, pat: P) -> &[u8];

	/// # Trim Start Matches.
	///
	/// Trim arbitrary leading bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
	///
	/// ```
	/// use trimothy::TrimSliceMatches;
	///
	/// let s: &[u8] = b"...Custom Trim!...";
	/// assert_eq!(s.trim_start_matches(b'.'), b"Custom Trim!...");
	/// assert_eq!(s.trim_start_matches([b'.']), b"Custom Trim!...");
	/// assert_eq!(s.trim_start_matches(&[b'.']), b"Custom Trim!...");
	/// assert_eq!(s.trim_start_matches(|b| b'.' == b), b"Custom Trim!...");
	/// ```
	fn trim_start_matches<P: MatchPattern<u8>>(&self, pat: P) -> &[u8];

	/// # Trim End Matches.
	///
	/// Trim arbitrary trailing bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
	///
	/// ```
	/// use trimothy::TrimSliceMatches;
	///
	/// let s: &[u8] = b"...Custom Trim!...";
	/// assert_eq!(s.trim_end_matches(b'.'), b"...Custom Trim!");
	/// assert_eq!(s.trim_end_matches([b'.']), b"...Custom Trim!");
	/// assert_eq!(s.trim_end_matches(&[b'.']), b"...Custom Trim!");
	/// assert_eq!(s.trim_end_matches(|b| b'.' == b), b"...Custom Trim!");
	/// ```
	fn trim_end_matches<P: MatchPattern<u8>>(&self, pat: P) -> &[u8];
}


/// # Helper: Trim Slice Matches.
macro_rules! trim_slice {
	($($ty:ty),+ $(,)?) => ($(
		impl TrimSliceMatches for $ty {
			/// # Trim Matches.
			///
			/// Trim arbitrary leading and trailing bytes as determined by the provided
			/// pattern, which can be:
			/// * A single `u8`;
			/// * An array or slice of `u8`;
			/// * A `&BTreeSet<u8>`;
			/// * A callback with the signature `Fn(u8) -> bool`;
			fn trim_matches<P: MatchPattern<u8>>(&self, pat: P) -> &[u8] {
				let mut src: &[u8] = &self;
				while let [first, rest @ ..] = src {
					if pat.is_match(*first) { src = rest; }
					else { break; }
				}

				while let [rest @ .., last] = src {
					if pat.is_match(*last) { src = rest; }
					else { break; }
				}
				src
			}

			/// # Trim Start Matches.
			///
			/// Trim arbitrary leading bytes as determined by the provided
			/// pattern, which can be:
			/// * A single `u8`;
			/// * An array or slice of `u8`;
			/// * A `&BTreeSet<u8>`;
			/// * A callback with the signature `Fn(u8) -> bool`;
			fn trim_start_matches<P: MatchPattern<u8>>(&self, pat: P) -> &[u8] {
				let mut src: &[u8] = &self;
				while let [first, rest @ ..] = src {
					if pat.is_match(*first) { src = rest; }
					else { break; }
				}
				src
			}

			/// # Trim Start Matches.
			///
			/// Trim arbitrary trailing bytes as determined by the provided
			/// pattern, which can be:
			/// * A single `u8`;
			/// * An array or slice of `u8`;
			/// * A `&BTreeSet<u8>`;
			/// * A callback with the signature `Fn(u8) -> bool`;
			fn trim_end_matches<P: MatchPattern<u8>>(&self, pat: P) -> &[u8] {
				let mut src: &[u8] = &self;
				while let [rest @ .., last] = src {
					if pat.is_match(*last) { src = rest; }
					else { break; }
				}
				src
			}
		}
	)+);
}

trim_slice!([u8], Box<[u8]>, Vec<u8>);



#[cfg(test)]
mod tests {
	use super::*;
	use alloc::collections::BTreeSet;
	use brunch as _;

	const T_EMPTY: &[u8] = b"";
	const T_HELLO: &[u8] = b"hello";
	const T_HELLO_E: &[u8] = b"hello\t";

	#[test]
	fn t_trim() {
		let tests: [(&str, &str); 6] = [
			("", ""),
			(" \t\n\r", ""),
			("hello", "hello"),
			("hello\t", "hello"),
			("\thello", "hello"),
			("\n  hello world! \t", "hello world!"),
		];

		for (raw, expected) in &tests {
			let a = raw.as_bytes();
			let b = expected.as_bytes();
			assert_eq!(a.trim_ascii(), b);

			let a = a.to_vec();
			assert_eq!(a.trim_ascii(), b);

			let a = a.into_boxed_slice();
			assert_eq!(a.trim_ascii(), b);
		}

		assert_eq!(T_EMPTY.trim_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(T_EMPTY.to_vec().trim_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from(T_EMPTY).trim_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(b"  ".trim_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(b"  ".to_vec().trim_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from("  ".as_bytes()).trim_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(T_HELLO_E.trim_matches(|b: u8| b'h' == b), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_matches(|b: u8| b'h' == b), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(|b: u8| b'h' == b), b"ello\t");

		assert_eq!(T_HELLO_E.trim_matches(b'h'), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_matches(b'h'), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(b'h'), b"ello\t");

		let set = BTreeSet::from([b'h']);
		assert_eq!(T_HELLO_E.trim_matches(&set), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_matches(&set), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(&set), b"ello\t");

		// This should also work on arrays.
		let arr: [u8; 5] = [b' ', b' ', b'.', b' ', b' '];
		assert_eq!(arr.trim_ascii(), &[b'.']);
	}

	#[test]
	fn t_trim_start() {
		let tests: [(&str, &str); 6] = [
			("", ""),
			(" \t\n\r", ""),
			("hello", "hello"),
			("hello\t", "hello\t"),
			("\thello", "hello"),
			("\n  hello world! \t", "hello world! \t"),
		];

		for (raw, expected) in &tests {
			let a = raw.as_bytes();
			let b = expected.as_bytes();
			assert_eq!(a.trim_ascii_start(), b);

			let a = a.to_vec();
			assert_eq!(a.trim_ascii_start(), b);

			let a = a.into_boxed_slice();
			assert_eq!(a.trim_ascii_start(), b);
		}

		assert_eq!(T_EMPTY.trim_start_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(T_EMPTY.to_vec().trim_start_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from(T_EMPTY).trim_start_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(b"  ".trim_start_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(b"  ".to_vec().trim_start_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from("  ".as_bytes()).trim_start_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(T_HELLO_E.trim_start_matches(|b: u8| b'h' == b), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_start_matches(|b: u8| b'h' == b), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_start_matches(|b: u8| b'h' == b), b"ello\t");

		assert_eq!(T_HELLO_E.trim_start_matches(b'h'), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_start_matches(b'h'), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_start_matches(b'h'), b"ello\t");

		let set = BTreeSet::from([b'h']);
		assert_eq!(T_HELLO_E.trim_start_matches(&set), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_start_matches(&set), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_start_matches(&set), b"ello\t");
	}

	#[test]
	fn t_trim_end() {
		let tests: [(&str, &str); 6] = [
			("", ""),
			(" \t\n\r", ""),
			("hello", "hello"),
			("hello\t", "hello"),
			("\thello", "\thello"),
			("\n  hello world! \t", "\n  hello world!"),
		];

		for (raw, expected) in &tests {
			let a = raw.as_bytes();
			let b = expected.as_bytes();
			assert_eq!(a.trim_ascii_end(), b);

			let a = a.to_vec();
			assert_eq!(a.trim_ascii_end(), b);

			let a = a.into_boxed_slice();
			assert_eq!(a.trim_ascii_end(), b);
		}

		assert_eq!(T_EMPTY.trim_end_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(T_EMPTY.to_vec().trim_end_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from(T_EMPTY).trim_end_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(b"  ".trim_end_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(b"  ".to_vec().trim_end_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from("  ".as_bytes()).trim_end_matches(|b: u8| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(T_HELLO_E.trim_matches(|b: u8| b'\t' == b), T_HELLO);
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(|b: u8| b'\t' == b), T_HELLO);
		assert_eq!(T_HELLO_E.to_vec().trim_matches(|b: u8| b'\t' == b), T_HELLO);

		assert_eq!(T_HELLO_E.trim_matches(b'\t'), T_HELLO);
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(b'\t'), T_HELLO);
		assert_eq!(T_HELLO_E.to_vec().trim_matches(b'\t'), T_HELLO);

		let set = BTreeSet::from([b'\t']);
		assert_eq!(T_HELLO_E.trim_matches(&set), T_HELLO);
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(&set), T_HELLO);
		assert_eq!(T_HELLO_E.to_vec().trim_matches(&set), T_HELLO);
	}
}
