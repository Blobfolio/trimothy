/*!
# Trimothy: Trim and (Maybe) Normalize.
*/

use alloc::{
	borrow::Cow,
	string::String,
	vec::Vec,
};
use crate::TrimMut;



/// # Trim and (Maybe) Normalize Whitespace.
///
/// This trait adds a single `trim_and_normalize` method to owned and borrowed
/// string and byte slices that trims leading/trailing whitespace, and
/// compacts/normalizes spans of _inner_ whitespace to a single horizontal
/// space.
///
/// In keeping with the rest of the library, "whitespace" here means
/// [`char::is_whitespace`] for string sources, and [`u8::is_ascii_whitespace`]
/// for byte sources.
///
/// ## Examples
///
/// ```
/// use trimothy::TrimNormal;
///
/// assert_eq!(
///     " H\r\nE\u{2001}L  \u{3000}\u{205f}L\tO  ".trim_and_normalize(),
///     "H E L L O",
/// );
/// ```
pub trait TrimNormal {
	/// # Output Type.
	type Normalized;

	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	fn trim_and_normalize(self) -> Self::Normalized;
}



/// # Trim and (Maybe) Normalize Whitespace: `char` Iterator Adapter.
///
/// This trait provides the equivalent of [`TrimNormal`] for arbitrary
/// iterators of `char`.
///
/// ## Examples
///
/// ```
/// use trimothy::TrimNormalChars;
///
/// let foo = " H E  L\r\nL O\n".chars()
///     .trim_and_normalize()
///     .collect::<String>();
/// assert_eq!(foo, "H E L L O");
/// ```
pub trait TrimNormalChars<I: Iterator<Item=char>> {
	/// # Trim and Normalize Whitespace: `char` Iterator Adapter.
	///
	/// Filter an `Iterator<Item=char>` to omit leading/trailing whitespace,
	/// and reduce inner spans of whitespace to single horizontal spaces.
	fn trim_and_normalize(self) -> TrimNormalIter<char, I>;
}

impl<I: Iterator<Item=char>> TrimNormalChars<I> for I {
	#[inline]
	/// # Trim and Normalize Whitespace.
	///
	/// Filter an `Iterator<Item=char>` to omit leading/trailing whitespace,
	/// and reduce inner spans of whitespace to single horizontal spaces.
	fn trim_and_normalize(mut self) -> TrimNormalIter<char, I> {
		// We can trim the start before, er, starting.
		let next = self.by_ref().find(|c| ! c.is_whitespace());
		TrimNormalIter { iter: self, next }
	}
}



/// # Trim and (Maybe) Normalize Whitespace: `u8` Iterator Adapter.
///
/// This trait provides the equivalent of [`TrimNormal`] for arbitrary
/// iterators of `u8`.
///
/// ## Examples
///
/// ```
/// use trimothy::TrimNormalBytes;
///
/// let foo = b" H E  L\r\nL O\n".iter()
///     .copied()
///     .trim_and_normalize()
///     .collect::<Vec<u8>>();
/// assert_eq!(foo, b"H E L L O");
/// ```
pub trait TrimNormalBytes<I: Iterator<Item=u8>> {
	/// # Trim and Normalize Whitespace: `u8` Iterator Adapter.
	///
	/// Filter an `Iterator<Item=u8>` to omit leading/trailing whitespace,
	/// and reduce inner spans of whitespace to single horizontal spaces.
	fn trim_and_normalize(self) -> TrimNormalIter<u8, I>;
}

impl<I: Iterator<Item=u8>> TrimNormalBytes<I> for I {
	#[inline]
	/// # Trim and Normalize Whitespace.
	///
	/// Filter an `Iterator<Item=u8>` to omit leading/trailing whitespace,
	/// and reduce inner spans of whitespace to single horizontal spaces.
	fn trim_and_normalize(mut self) -> TrimNormalIter<u8, I> {
		// We can trim the start before, er, starting.
		let next = self.by_ref().find(|c| ! c.is_ascii_whitespace());
		TrimNormalIter { iter: self, next }
	}
}



#[derive(Debug, Clone)]
/// # Iterator for [`TrimNormalBytes`] and [`TrimNormalChars`].
///
/// This struct is yielded by [`TrimNormalBytes::trim_and_normalize`] and
/// [`TrimNormalChars::trim_and_normalize`].
///
/// Refer to their documentation for more details.
pub struct TrimNormalIter<T: Copy + Sized, I: Iterator<Item=T>> {
	/// # The Iterator.
	iter: I,

	/// # Next Buffer.
	///
	/// Sometimes we need to look ahead, and sometimes we need to save what we
	/// find there for the next cycle.
	next: Option<T>,
}

/// # Helper: Iteration.
///
/// The `char` and `u8` implementations work _almost_ exactly the same way!
macro_rules! iter {
	($ty:ty, $space:literal, $cmp:ident) => (
		impl<I: Iterator<Item=$ty>> Iterator for TrimNormalIter<$ty, I> {
			type Item = $ty;

			fn next(&mut self) -> Option<Self::Item> {
				// If we have something in the buffer, return it.
				if let Some(next) = self.next.take() { return Some(next); }

				// Pull the next thing.
				let next = self.iter.next()?;

				// Normalization required?
				if next.$cmp() {
					// Fast-forward to the next non-whitespace.
					self.next = self.iter.by_ref().find(|c| ! c.$cmp());
					if self.next.is_some() { Some($space) }
					else { None }
				}
				// Return it as-is.
				else { Some(next) }
			}

			fn size_hint(&self) -> (usize, Option<usize>) {
				let lower = usize::from(self.next.is_some()); // Definitely.
				let (_, upper) = self.iter.size_hint();       // Maybe.
				(lower, upper.map(|n| n + lower))
			}
		}
	);
}

iter!(char, ' ', is_whitespace);
iter!(u8, b' ', is_ascii_whitespace);



impl<'a> TrimNormal for &'a str {
	/// # Output Type.
	type Normalized = Cow<'a, str>;

	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimNormal;
	///
	/// const ABNORMAL: &str = " H\r\nE\u{2001}L  \u{3000}\u{205f}L\tO  ";
	///
	/// assert_eq!(
	///     ABNORMAL.trim_and_normalize(),
	///     "H E L L O",
	/// );
	///
	/// // The above will have had to allocate to work its magic:
	/// assert!(matches!(
	///     ABNORMAL.trim_and_normalize(),
	///     Cow::Owned(_),
	/// ));
	///
	/// // But in other cases that might not be necessary.
	/// assert!(matches!(
	///     " Edges Trimmed Free\n\n".trim_and_normalize(),
	///     Cow::Borrowed(_),
	/// ));
	/// ```
	fn trim_and_normalize(self) -> Self::Normalized {
		// Trim leading/trailing whitespace to make life easier on ourselves.
		let src = self.trim();

		// Run through what we've got, checking to see if it matches up to the
		// original.
		let mut len = 0;
		let mut ws = true;
		let mut iter = src.chars();
		while let Some(c) = iter.next() {
			let mut change = None;
			if c.is_whitespace() {
				// Redundant inner whitespace; need to strip!
				if ws { change.replace(false); }
				else {
					ws = true;
					// Weird inner whitespace; need to replace!
					if c != ' ' { change.replace(true); }
				}
			}
			else { ws = false; }

			// The source is no good; we'll have to build a new string.
			if let Some(change) = change {
				// No need to overthink the capacity.
				let mut out = String::with_capacity(src.len());

				// Copy over the good parts en masse, if any.
				if len != 0 { out.push_str(&src[..len]); }

				// Push a space if needed.
				if change { out.push(' '); }

				// Run through the remainder, char-by-char, dropping/altering
				// on-the-fly.
				out.extend(iter.filter_map(|c|
					if c.is_whitespace() {
						if ws { None }
						else {
							ws = true;
							Some(' ')
						}
					}
					else {
						ws = false;
						Some(c)
					}
				));

				// Done!
				return Cow::Owned(out);
			}

			// Move the stop past this character.
			len += c.len_utf8();
		}

		// It was fine!
		Cow::Borrowed(&src[..len])
	}
}

impl TrimNormal for Cow<'_, str> {
	/// # Output Type.
	type Normalized = Self;

	#[inline]
	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimNormal;
	///
	/// assert_eq!(
	///     Cow::Borrowed(" H\r\nE\u{2001}L  \u{3000}\u{205f}L\tO  ")
	///         .trim_and_normalize(),
	///     "H E L L O",
	/// );
	/// ```
	fn trim_and_normalize(self) -> Self::Normalized {
		match self {
			Cow::Borrowed(s) => s.trim_and_normalize(),
			Cow::Owned(s) => Cow::Owned(s.trim_and_normalize()),
		}
	}
}

impl TrimNormal for &mut String {
	/// # Output Type.
	type Normalized = Self;

	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimNormal;
	///
	/// /// A Contrived Example…
	/// fn fix_whitespace(src: &mut String) -> bool {
	///     src.trim_and_normalize();
	///     ! src.is_empty()
	/// }
	///
	/// let mut abnormal = String::new();
	/// abnormal.push_str(" H\r\n");
	/// abnormal.push_str("E\u{2001}");
	/// abnormal.push_str("L  \u{3000}\u{205f}L\tO  ");
	///
	/// assert!(fix_whitespace(&mut abnormal));
	/// assert_eq!(abnormal, "H E L L O");
	/// ```
	fn trim_and_normalize(self) -> Self::Normalized {
		// Trim the trailing whitespace.
		self.trim_end_mut();

		// Now trim the beginning and inner whitespace.
		let mut ws = true;
		let mut other = 0;
		self.retain(|v|
			if v.is_whitespace() {
				if ws { false }
				else {
					ws = true;
					if v != ' ' { other += 1; } // We'll need a second pass.
					true
				}
			}
			else {
				ws = false;
				true
			}
		);

		// If any non-space whitespace remains, we'll need to loop back through
		// and swap them out with regular spaces.
		let mut end = self.len();
		while 0 < other {
			let mut len = 0;
			if let Some(pos) = self[..end].rfind(|c: char|
				if c.is_whitespace() && c != ' ' {
					len = c.len_utf8(); // Number of bytes to replace.
					true
				}
				else { false }
			) {
				self.replace_range(pos..pos + len, " ");
				end = pos; // Don't retread parts we've already looked at.
				other -= 1;
			}
			else { break; }
		}

		// Done!
		self
	}
}

impl<'a> TrimNormal for &'a String {
	/// # Output Type.
	type Normalized = Cow<'a, str>;

	#[inline]
	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimNormal;
	///
	/// // If for some reason you don't want the original value to be
	/// // replaced, you can trim/normalize a reference instead:
	/// let abnormal = String::from(" H\r\nE\u{2001}L  \u{3000}\u{205f}L\tO  ");
	/// let normal = (&abnormal).trim_and_normalize();
	///
	/// assert_ne!(abnormal, normal);
	/// assert_eq!(normal, "H E L L O");
	/// ```
	fn trim_and_normalize(self) -> Self::Normalized {
		<&str as TrimNormal>::trim_and_normalize(self.as_str())
	}
}

impl TrimNormal for String {
	/// # Output Type.
	type Normalized = Self;

	#[inline]
	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimNormal;
	///
	/// // A contrived example…
	/// let mut abnormal = String::new();
	/// abnormal.push_str(" H\r\n");
	/// abnormal.push_str("E\u{2001}");
	/// abnormal.push_str("L  \u{3000}\u{205f}L\tO  ");
	///
	/// abnormal = abnormal.trim_and_normalize();
	/// assert_eq!(abnormal, "H E L L O");
	/// ```
	fn trim_and_normalize(mut self) -> Self::Normalized {
		<&mut Self as TrimNormal>::trim_and_normalize(&mut self);
		self
	}
}



impl<'a> TrimNormal for &'a [u8] {
	/// # Output Type.
	type Normalized = Cow<'a, [u8]>;

	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimNormal;
	///
	/// const ABNORMAL: &[u8] = b" H\r\nE L  \t\x0CL\tO  ";
	///
	/// assert_eq!(
	///     ABNORMAL.trim_and_normalize().as_ref(),
	///     b"H E L L O",
	/// );
	///
	/// // The above will have had to allocate to work its magic:
	/// assert!(matches!(
	///     ABNORMAL.trim_and_normalize(),
	///     Cow::Owned(_),
	/// ));
	///
	/// // But in other cases that might not be necessary.
	/// assert!(matches!(
	///     b" Edges Trimmed Free\n\n".trim_and_normalize(),
	///     Cow::Borrowed(_),
	/// ));
	/// ```
	fn trim_and_normalize(self) -> Self::Normalized {
		// Trim leading/trailing whitespace to make life easier on ourselves.
		let src = self.trim_ascii();

		// Run through what we've got, checking to see if it matches up to the
		// original.
		let mut len = 0;
		let mut ws = true;
		let mut iter = src.iter().copied();
		while let Some(c) = iter.next() {
			let mut change = None;
			if c.is_ascii_whitespace() {
				// Redundant inner whitespace; need to strip!
				if ws { change.replace(false); }
				else {
					ws = true;
					// Weird inner whitespace; need to replace!
					if c != b' ' { change.replace(true); }
				}
			}
			else { ws = false; }

			// The source is no good; we'll have to build a new string.
			if let Some(change) = change {
				// No need to overthink the capacity.
				let mut out = Vec::<u8>::with_capacity(src.len());

				// Copy over the good parts en masse, if any.
				if len != 0 { out.extend_from_slice(&src[..len]); }

				// Push a space if needed.
				if change { out.push(b' '); }

				// Run through the remainder, char-by-char, dropping/altering
				// on-the-fly.
				out.extend(iter.filter_map(|c|
					if c.is_ascii_whitespace() {
						if ws { None }
						else {
							ws = true;
							Some(b' ')
						}
					}
					else {
						ws = false;
						Some(c)
					}
				));

				// Done!
				return Cow::Owned(out);
			}

			// Move the stop past this character.
			len += 1;
		}

		// It was fine!
		Cow::Borrowed(&src[..len])
	}
}

impl TrimNormal for Cow<'_, [u8]> {
	/// # Output Type.
	type Normalized = Self;

	#[inline]
	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimNormal;
	///
	/// assert_eq!(
	///     Cow::Borrowed(b" H\r\nE L  \t\x0CL\tO  ")
	///         .trim_and_normalize()
	///         .as_ref(),
	///     b"H E L L O",
	/// );
	/// ```
	fn trim_and_normalize(self) -> Self::Normalized {
		match self {
			Cow::Borrowed(s) => s.trim_and_normalize(),
			Cow::Owned(s) => Cow::Owned(s.trim_and_normalize()),
		}
	}
}

impl TrimNormal for &mut Vec<u8> {
	/// # Output Type.
	type Normalized = Self;

	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimNormal;
	///
	/// /// A Contrived Example…
	/// fn fix_whitespace(src: &mut Vec<u8>) -> bool {
	///     src.trim_and_normalize();
	///     ! src.is_empty()
	/// }
	///
	/// let mut abnormal = Vec::<u8>::new();
	/// abnormal.extend_from_slice(b" H\r\n");
	/// abnormal.extend_from_slice(b"E ");
	/// abnormal.extend_from_slice(b"L  \nL\tO  ");
	///
	/// assert!(fix_whitespace(&mut abnormal));
	/// assert_eq!(abnormal, b"H E L L O");
	/// ```
	fn trim_and_normalize(self) -> Self::Normalized {
		// Trim the beginning and normalize the rest.
		let mut ws = true;
		self.retain_mut(|v|
			if v.is_ascii_whitespace() {
				if ws { false }
				else {
					ws = true;
					*v = b' ';
					true
				}
			}
			else {
				ws = false;
				true
			}
		);

		// Trim the end, if needed.
		if ws { self.trim_end_mut(); }

		self
	}
}

impl TrimNormal for Vec<u8> {
	/// # Output Type.
	type Normalized = Self;

	#[inline]
	/// # Trim and Normalize Whitespace.
	///
	/// Trim the leading/trailing whitespace, and compact/normalize spans of
	/// _inner_ whitespace to a single horizontal space.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimNormal;
	///
	/// // A contrived example…
	/// let mut abnormal = Vec::<u8>::new();
	/// abnormal.extend_from_slice(b" H\r\n");
	/// abnormal.extend_from_slice(b"E ");
	/// abnormal.extend_from_slice(b"L  \nL\tO  ");
	///
	/// abnormal = abnormal.trim_and_normalize();
	/// assert_eq!(abnormal, b"H E L L O");
	/// ```
	fn trim_and_normalize(mut self) -> Self::Normalized {
		<&mut Self as TrimNormal>::trim_and_normalize(&mut self);
		self
	}
}



#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn trim_and_normalize_borrowed() {
		// These should all be salvageable.
		for (raw, expected) in [
			("", ""),
			("  ", ""),
			("\n\r\x0C  H E L L O\t\t", "H E L L O"),
		] {
			// &str.
			let normal = raw.trim_and_normalize();
			assert_eq!(normal, expected);
			assert!(matches!(normal, Cow::Borrowed(_)));

			// &[u8].
			let normal = raw.as_bytes().trim_and_normalize();
			assert_eq!(normal, expected.as_bytes());
			assert!(matches!(normal, Cow::Borrowed(_)));

			// Test the owned versions just for fun.
			let normal: String = String::from(raw).trim_and_normalize();
			assert_eq!(normal, expected);

			let normal: Vec<u8> = raw.as_bytes().to_vec().trim_and_normalize();
			assert_eq!(normal, expected.as_bytes());

			// Test the iterators too.
			let normal: String = raw.chars().trim_and_normalize().collect();
			assert_eq!(normal, expected);

			let normal: Vec<u8> = raw.bytes().trim_and_normalize().collect();
			assert_eq!(normal, expected.as_bytes());
		}

		// Strings check a bit more.
		for (raw, expected) in [
			("\u{2003}", ""),
			("\u{2003}\u{2003}HEL LO\r\u{2003}", "HEL LO"),
		] {
			// &str.
			let normal = raw.trim_and_normalize();
			assert_eq!(normal, expected);
			assert!(matches!(normal, Cow::Borrowed(_)));

			// String.
			let normal: String = String::from(raw).trim_and_normalize();
			assert_eq!(normal, expected);

			// Iterator.
			let normal: String = raw.chars().trim_and_normalize().collect();
			assert_eq!(normal, expected);
		}

		// All the whitespace!
		let sandwich = core::iter::once('[')
			.chain(('\0'..=char::MAX).filter(|c| c.is_whitespace()))
			.chain(core::iter::once(']'))
			.collect::<String>();
		assert_eq!(sandwich.as_str().trim_and_normalize(), "[ ]");
		assert_eq!(sandwich.trim_and_normalize(), "[ ]");

		// And the iterator.
		let sandwich = core::iter::once('[')
			.chain(('\0'..=char::MAX).filter(|c| c.is_whitespace()))
			.chain(core::iter::once(']'))
			.trim_and_normalize()
			.collect::<String>();
		assert_eq!(sandwich, "[ ]");
	}

	#[test]
	fn trim_and_normalize_owned() {
		// These require allocation.
		for (raw, expected) in [
			("H  I", "H I"),
			("H\tI", "H I"),
			("H\tE  L\n\rL\x0CO ", "H E L L O"),
		] {
			// &str.
			let normal = raw.trim_and_normalize();
			assert_eq!(normal, expected);
			assert!(matches!(normal, Cow::Owned(_)));

			// &[u8].
			let normal = raw.as_bytes().trim_and_normalize();
			assert_eq!(normal, expected.as_bytes());
			assert!(matches!(normal, Cow::Owned(_)));

			// Test the owned versions just for fun.
			let normal: String = String::from(raw).trim_and_normalize();
			assert_eq!(normal, expected);

			let normal: Vec<u8> = raw.as_bytes().to_vec().trim_and_normalize();
			assert_eq!(normal, expected.as_bytes());

			// Test the iterators too.
			let normal: String = raw.chars().trim_and_normalize().collect();
			assert_eq!(normal, expected);

			let normal: Vec<u8> = raw.bytes().trim_and_normalize().collect();
			assert_eq!(normal, expected.as_bytes());
		}

		// Strings check a bit more.
		for (raw, expected) in [
			("H\u{2003}I", "H I"),
			("\u{2003}\u{2003}HEL\u{2003} LO\r\u{2003}", "HEL LO"),
		] {
			// &str.
			let normal = raw.trim_and_normalize();
			assert_eq!(normal, expected);
			assert!(matches!(normal, Cow::Owned(_)));

			// String.
			let normal: String = String::from(raw).trim_and_normalize();
			assert_eq!(normal, expected);

			// Iterator.
			let normal: String = raw.chars().trim_and_normalize().collect();
			assert_eq!(normal, expected);
		}
	}
}
