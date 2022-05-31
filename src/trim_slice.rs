/*!
# Trimothy - Trim Slice
*/

use alloc::{
	boxed::Box,
	vec::Vec,
};



/// # Trim Slice.
///
/// The [`TrimSlice`] trait brings basic trimming support to `&[u8]`,
/// `Vec<u8>`, and `Box<[u8]>` types, very similar to the ones enjoyed by
/// `String`/`&str`.
///
/// The trait methods included are:
///
/// | Method | Description |
/// | ------ | ----------- |
/// | `trim` | Trim leading and trailing (ASCII) whitespace. |
/// | `trim_start` | Trim leading (ASCII) whitespace. |
/// | `trim_end` | Trim trailing (ASCII) whitespace. |
///
/// **Note:** Because these methods work with individual bytes — rather than chars
/// — these methods only trim [`u8::is_ascii_whitespace`], not [`char::is_whitespace`].
pub trait TrimSlice {
	/// # Trim.
	///
	/// Trim leading and trailing (ASCII) whitespace from a slice.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimSlice;
	///
	/// let s: &[u8] = b" This is a slice with whitespace on the ends.\n";
	/// assert_eq!(s.trim(), b"This is a slice with whitespace on the ends.");
	/// ```
	fn trim(&self) -> &[u8];

	/// # Trim Start.
	///
	/// Trim leading (ASCII) whitespace from a slice.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimSlice;
	///
	/// let s: &[u8] = b" This is a slice with whitespace on the ends.\n";
	/// assert_eq!(s.trim_start(), b"This is a slice with whitespace on the ends.\n");
	/// ```
	fn trim_start(&self) -> &[u8];

	/// # Trim End.
	///
	/// Trim trailing (ASCII) whitespace from a slice.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimSlice;
	///
	/// let s: &[u8] = b" This is a slice with whitespace on the ends.\n";
	/// assert_eq!(s.trim_end(), b" This is a slice with whitespace on the ends.");
	/// ```
	fn trim_end(&self) -> &[u8];
}



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
/// | `trim_matches` | Trim arbitrary leading and trailing bytes via callback. |
/// | `trim_start_matches` | Trim arbitrary leading bytes via callback. |
/// | `trim_end_matches` | Trim arbitrary trailing bytes via callback. |
pub trait TrimSliceMatches {
	/// # Trim Matches.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// callback, where a return value of `true` means trim.
	///
	/// ```
	/// use trimothy::TrimSliceMatches;
	///
	/// let s: &[u8] = b"...Custom Trim!...";
	/// assert_eq!(s.trim_matches(|b| b'.' == b), b"Custom Trim!");
	/// ```
	fn trim_matches<F>(&self, cb: F) -> &[u8]
	where F: Fn(u8) -> bool;

	/// # Trim Start Matches.
	///
	/// Trim arbitrary leading bytes as determined by the provided callback,
	/// where a return value of `true` means trim.
	///
	/// ```
	/// use trimothy::TrimSliceMatches;
	///
	/// let s: &[u8] = b"...Custom Trim!...";
	/// assert_eq!(s.trim_start_matches(|b| b'.' == b), b"Custom Trim!...");
	/// ```
	fn trim_start_matches<F>(&self, cb: F) -> &[u8]
	where F: Fn(u8) -> bool;

	/// # Trim End Matches.
	///
	/// Trim arbitrary trailing bytes as determined by the provided callback,
	/// where a return value of `true` means trim.
	///
	/// ```
	/// use trimothy::TrimSliceMatches;
	///
	/// let s: &[u8] = b"...Custom Trim!...";
	/// assert_eq!(s.trim_end_matches(|b| b'.' == b), b"...Custom Trim!");
	/// ```
	fn trim_end_matches<F>(&self, cb: F) -> &[u8]
	where F: Fn(u8) -> bool;
}


macro_rules! trim_slice {
	($($ty:ty),+ $(,)?) => ($(
		impl TrimSlice for $ty {
			/// # Trim.
			///
			/// Trim leading and trailing (ASCII) whitespace from a slice.
			fn trim(&self) -> &[u8] { trim_end(trim_start(&self)) }

			#[inline]
			/// # Trim Start.
			///
			/// Trim leading (ASCII) whitespace from a slice.
			fn trim_start(&self) -> &[u8] { trim_start(&self) }

			#[inline]
			/// # Trim End.
			///
			/// Trim trailing (ASCII) whitespace from a slice.
			fn trim_end(&self) -> &[u8] { trim_end(&self) }
		}

		impl TrimSliceMatches for $ty {
			/// # Trim Matches.
			///
			/// Trim arbitrary leading and trailing bytes as determined by the provided
			/// callback, where a return value of `true` means trim.
			fn trim_matches<F>(&self, cb: F) -> &[u8]
			where F: Fn(u8) -> bool {
				let cb = |b: &u8| ! cb(*b);

				self.iter()
					.position(cb)
					.map_or(&[], |start| {
						// We know there is an end because there's a beginning.
						let end = self.iter().rposition(cb).unwrap();
						&self[start..=end]
					})
			}

			/// # Trim Start Matches.
			///
			/// Trim arbitrary leading bytes as determined by the provided callback,
			/// where a return value of `true` means trim.
			fn trim_start_matches<F>(&self, cb: F) -> &[u8]
			where F: Fn(u8) -> bool {
				self.iter()
					.position(|b: &u8| ! cb(*b))
					.map_or(&[], |p| &self[p..])
			}

			/// # Trim Start Matches.
			///
			/// Trim arbitrary leading bytes as determined by the provided callback,
			/// where a return value of `true` means trim.
			fn trim_end_matches<F>(&self, cb: F) -> &[u8]
			where F: Fn(u8) -> bool {
				self.iter()
					.rposition(|b: &u8| ! cb(*b))
					.map_or(&[], |p| &self[..=p])
			}
		}
	)+);
}

trim_slice!([u8], Box<[u8]>, Vec<u8>);



/// # Trim Slice Start.
///
/// This is a copy of the nightly `trim_ascii_start` so it can be used on
/// stable. If/when that feature is stabilized, we'll use it directly.
const fn trim_start(mut src: &[u8]) -> &[u8] {
	while let [first, rest @ ..] = src {
		if first.is_ascii_whitespace() { src = rest; }
		else { break; }
	}
	src
}

/// # Trim Slice End.
///
/// This is a copy of the nightly `trim_ascii_end` so it can be used on
/// stable. If/when that feature is stabilized, we'll use it directly.
const fn trim_end(mut src: &[u8]) -> &[u8] {
	while let [rest @ .., last] = src {
		if last.is_ascii_whitespace() { src = rest; }
		else { break; }
	}
	src
}



#[cfg(test)]
mod tests {
	use super::*;
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

		for (raw, expected) in tests.iter() {
			let a = raw.as_bytes();
			let b = expected.as_bytes();
			assert_eq!(a.trim(), b);

			let a = a.to_vec();
			assert_eq!(a.trim(), b);

			let a = a.into_boxed_slice();
			assert_eq!(a.trim(), b);
		}

		assert_eq!(T_EMPTY.trim_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(T_EMPTY.to_vec().trim_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from(T_EMPTY).trim_matches(|b| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!("  ".as_bytes().trim_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!("  ".as_bytes().to_vec().trim_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from("  ".as_bytes()).trim_matches(|b| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(T_HELLO_E.trim_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(|b| b'h' == b), b"ello\t");

		// This should also work on arrays.
		let arr: [u8; 5] = [b' ', b' ', b'.', b' ', b' '];
		assert_eq!(arr.trim(), &[b'.']);
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

		for (raw, expected) in tests.iter() {
			let a = raw.as_bytes();
			let b = expected.as_bytes();
			assert_eq!(a.trim_start(), b);

			let a = a.to_vec();
			assert_eq!(a.trim_start(), b);

			let a = a.into_boxed_slice();
			assert_eq!(a.trim_start(), b);
		}

		assert_eq!(T_EMPTY.trim_start_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(T_EMPTY.to_vec().trim_start_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from(T_EMPTY).trim_start_matches(|b| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!("  ".as_bytes().trim_start_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!("  ".as_bytes().to_vec().trim_start_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from("  ".as_bytes()).trim_start_matches(|b| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(T_HELLO_E.trim_start_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_start_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_start_matches(|b| b'h' == b), b"ello\t");
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

		for (raw, expected) in tests.iter() {
			let a = raw.as_bytes();
			let b = expected.as_bytes();
			assert_eq!(a.trim_end(), b);

			let a = a.to_vec();
			assert_eq!(a.trim_end(), b);

			let a = a.into_boxed_slice();
			assert_eq!(a.trim_end(), b);
		}

		assert_eq!(T_EMPTY.trim_end_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(T_EMPTY.to_vec().trim_end_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from(T_EMPTY).trim_end_matches(|b| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!("  ".as_bytes().trim_end_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!("  ".as_bytes().to_vec().trim_end_matches(|b| b.is_ascii_whitespace()), T_EMPTY);
		assert_eq!(Box::<[u8]>::from("  ".as_bytes()).trim_end_matches(|b| b.is_ascii_whitespace()), T_EMPTY);

		assert_eq!(T_HELLO_E.trim_matches(|b| b'\t' == b), T_HELLO);
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(|b| b'\t' == b), T_HELLO);
		assert_eq!(T_HELLO_E.to_vec().trim_matches(|b| b'\t' == b), T_HELLO);
	}
}
