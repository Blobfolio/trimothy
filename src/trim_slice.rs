/*!
# Trimothy - Trim Slice
*/

#[cfg(not(feature = "std"))]
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
/// | Method | Description |
/// | ------ | ----------- |
/// | `trim` | Trim leading and trailing (ASCII) whitespace. |
/// | `trim_start` | Trim leading (ASCII) whitespace. |
/// | `trim_end` | Trim trailing (ASCII) whitespace. |
///
/// **Note:** because these methods work with individual bytes — rather than chars
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



impl TrimSlice for &[u8] {
	/// # Trim.
	///
	/// Trim leading and trailing (ASCII) whitespace from a slice.
	fn trim(&self) -> &[u8] {
		let start: usize = self.iter()
			.position(|b| ! b.is_ascii_whitespace())
			.unwrap_or(0);

		self.iter()
			.rposition(|b| ! b.is_ascii_whitespace())
			.map_or_else(|| &self[start..], |p| &self[start..=p])
	}

	/// # Trim Start.
	///
	/// Trim leading (ASCII) whitespace from a slice.
	fn trim_start(&self) -> &[u8] {
		self.iter()
			.position(|b| ! b.is_ascii_whitespace())
			.map_or(self, |p| &self[p..])
	}

	/// # Trim End.
	///
	/// Trim trailing (ASCII) whitespace from a slice.
	fn trim_end(&self) -> &[u8] {
		self.iter()
			.rposition(|b| ! b.is_ascii_whitespace())
			.map_or(self, |p| &self[..=p])
	}
}

impl TrimSliceMatches for &[u8] {
	/// # Trim Matches.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// callback, where a return value of `true` means trim.
	fn trim_matches<F>(&self, cb: F) -> &[u8]
	where F: Fn(u8) -> bool {
		let cb = |b: &u8| ! cb(*b);

		let start: usize = self.iter()
			.position(cb)
			.unwrap_or(0);

		self.iter()
			.rposition(cb)
			.map_or_else(|| &self[start..], |p| &self[start..=p])
	}

	/// # Trim Start Matches.
	///
	/// Trim arbitrary leading bytes as determined by the provided callback,
	/// where a return value of `true` means trim.
	fn trim_start_matches<F>(&self, cb: F) -> &[u8]
	where F: Fn(u8) -> bool {
		self.iter()
			.position(|b: &u8| ! cb(*b))
			.map_or(self, |p| &self[p..])
	}

	/// # Trim Start Matches.
	///
	/// Trim arbitrary leading bytes as determined by the provided callback,
	/// where a return value of `true` means trim.
	fn trim_end_matches<F>(&self, cb: F) -> &[u8]
	where F: Fn(u8) -> bool {
		self.iter()
			.rposition(|b: &u8| ! cb(*b))
			.map_or(self, |p| &self[..=p])
	}
}

macro_rules! trim_slice_alloc {
	($($ty:ty),+ $(,)?) => ($(
		impl TrimSlice for $ty {
			/// # Trim.
			///
			/// Trim leading and trailing (ASCII) whitespace from a slice.
			fn trim(&self) -> &[u8] {
				let start: usize = self.iter()
					.position(|b| ! b.is_ascii_whitespace())
					.unwrap_or(0);

				self.iter()
					.rposition(|b| ! b.is_ascii_whitespace())
					.map_or_else(|| &self[start..], |p| &self[start..=p])
			}

			/// # Trim Start.
			///
			/// Trim leading (ASCII) whitespace from a slice.
			fn trim_start(&self) -> &[u8] {
				self.iter()
					.position(|b| ! b.is_ascii_whitespace())
					.map_or(&self, |p| &self[p..])
			}

			/// # Trim End.
			///
			/// Trim trailing (ASCII) whitespace from a slice.
			fn trim_end(&self) -> &[u8] {
				self.iter()
					.rposition(|b| ! b.is_ascii_whitespace())
					.map_or(&self, |p| &self[..=p])
			}
		}

		impl TrimSliceMatches for $ty {
			/// # Trim Matches.
			///
			/// Trim arbitrary leading and trailing bytes as determined by the provided
			/// callback, where a return value of `true` means trim.
			fn trim_matches<F>(&self, cb: F) -> &[u8]
			where F: Fn(u8) -> bool {
				let cb = |b: &u8| ! cb(*b);

				let start: usize = self.iter()
					.position(cb)
					.unwrap_or(0);

				self.iter()
					.rposition(cb)
					.map_or_else(|| &self[start..], |p| &self[start..=p])
			}

			/// # Trim Start Matches.
			///
			/// Trim arbitrary leading bytes as determined by the provided callback,
			/// where a return value of `true` means trim.
			fn trim_start_matches<F>(&self, cb: F) -> &[u8]
			where F: Fn(u8) -> bool {
				self.iter()
					.position(|b: &u8| ! cb(*b))
					.map_or(&self, |p| &self[p..])
			}

			/// # Trim Start Matches.
			///
			/// Trim arbitrary leading bytes as determined by the provided callback,
			/// where a return value of `true` means trim.
			fn trim_end_matches<F>(&self, cb: F) -> &[u8]
			where F: Fn(u8) -> bool {
				self.iter()
					.rposition(|b: &u8| ! cb(*b))
					.map_or(&self, |p| &self[..=p])
			}
		}
	)+);
}

trim_slice_alloc!(Box<[u8]>, Vec<u8>);



#[cfg(test)]
mod tests {
	use super::*;
	use brunch as _;

	const T_EMPTY: &[u8] = b"";
	const T_HELLO: &[u8] = b"hello";
	const T_HELLO_E: &[u8] = b"hello\t";
	const T_HELLO_S: &[u8] = b"\thello";
	const T_HELLO_SE: &[u8] = b"\n  hello \t";

	#[test]
	fn t_trim() {
		let tests: &[(&[u8], &[u8])] = &[
			(T_EMPTY, T_EMPTY),
			(T_HELLO, T_HELLO),
			(T_HELLO_S, T_HELLO),
			(T_HELLO_E, T_HELLO),
			(T_HELLO_SE, T_HELLO),
		];

		for &(src, expected) in tests.iter() {
			assert_eq!(src.trim(), expected);
			assert_eq!(Box::<[u8]>::from(src).trim(), expected);
			assert_eq!(src.to_vec().trim(), expected);
		}

		assert_eq!(T_HELLO_E.trim_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_matches(|b| b'h' == b), b"ello\t");
	}

	#[test]
	fn t_trim_start() {
		let tests: &[(&[u8], &[u8])] = &[
			(T_EMPTY, T_EMPTY),
			(T_HELLO, T_HELLO),
			(T_HELLO_S, T_HELLO),
			(T_HELLO_E, T_HELLO_E),
			(T_HELLO_SE, b"hello \t"),
		];

		for &(src, expected) in tests.iter() {
			assert_eq!(src.trim_start(), expected);
			assert_eq!(Box::<[u8]>::from(src).trim_start(), expected);
			assert_eq!(src.to_vec().trim_start(), expected);
		}

		assert_eq!(T_HELLO_E.trim_start_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_start_matches(|b| b'h' == b), b"ello\t");
		assert_eq!(T_HELLO_E.to_vec().trim_start_matches(|b| b'h' == b), b"ello\t");
	}

	#[test]
	fn t_trim_end() {
		let tests: &[(&[u8], &[u8])] = &[
			(T_EMPTY, T_EMPTY),
			(T_HELLO, T_HELLO),
			(T_HELLO_S, T_HELLO_S),
			(T_HELLO_E, T_HELLO),
			(T_HELLO_SE, b"\n  hello"),
		];

		for &(src, expected) in tests.iter() {
			assert_eq!(src.trim_end(), expected);
			assert_eq!(Box::<[u8]>::from(src).trim_end(), expected);
			assert_eq!(src.to_vec().trim_end(), expected);
		}

		assert_eq!(T_HELLO_E.trim_matches(|b| b'\t' == b), T_HELLO);
		assert_eq!(Box::<[u8]>::from(T_HELLO_E).trim_matches(|b| b'\t' == b), T_HELLO);
		assert_eq!(T_HELLO_E.to_vec().trim_matches(|b| b'\t' == b), T_HELLO);
	}
}
