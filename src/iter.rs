/*!
# Trimothy - Normalized Whitespace Iterator
*/

use core::{
	iter::{
		Copied,
		Iterator,
	},
	slice::Iter,
	str::Chars,
};



/// # Normalized Whitespace Iterator.
///
/// This trait exposes a `normalized_whitespace` method that returns an
/// iterator over a byte or string slice that normalizes the whitespace, both
/// trimming the edges and compacting any inner whitespace spans, converting
/// them to single horizontal spaces (one per span).
///
/// To trim/compact control characters too, use the
/// `normalized_control_and_whitespace` method instead.
///
/// This can be called on an `&[u8]` or `&str` directly, or any iterator
/// yielding owned `u8` or `char` items.
///
/// Normalization can optionally be extended to cover control characters too,
/// trimming and compacting them as if they were whitespace (along with any
/// actual whitespace).
///
/// ```
/// use trimothy::NormalizeWhitespace;
///
/// let abnormal = "  \0Hello\0\t\0Dolly\0\0";
///
/// // Normally, crap like \0 won't get normalized.
/// let normal: String = abnormal.normalized_whitespace().collect();
/// assert_eq!(normal, "\0Hello\0 \0Dolly\0\0");
///
/// // But it can be.
/// let normal: String = abnormal.normalized_control_and_whitespace().collect();
/// assert_eq!(normal, "Hello Dolly");
/// ```
pub trait NormalizeWhitespace<T: Copy + Sized, I: Iterator<Item=T>> {
	/// # Normalized Whitespace Iterator.
	///
	/// Modify a byte or char iterator to trim the ends, and convert all
	/// contiguous inner whitespace to a single horizontal space.
	fn normalized_whitespace(self) -> NormalizeWhiteSpaceIter<T, I>;

	/// # Normalized Control/Whitespace Iterator.
	///
	/// Same as `normalized_whitespace`, but also trim/normalize control
	/// characters.
	fn normalized_control_and_whitespace(self) -> NormalizeWhiteSpaceIter<T, I>;
}

impl<'a> NormalizeWhitespace<u8, Copied<Iter<'a, u8>>> for &'a [u8] {
	/// # Normalized Whitespace Iterator.
	///
	/// Return an iterator over the byte/char contents with the edges trimmed,
	/// and all contiguous inner whitespace converted to a single horizontal
	/// space.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::NormalizeWhitespace;
	///
	/// let abnormal: &[u8] = b" Hello   World!\n";
	/// let normal: Vec<u8> = abnormal.normalized_whitespace().collect();
	/// assert_eq!(normal, b"Hello World!");
	/// ```
	///
	/// This'll work on `u8` iterators too. For example, if you wanted to
	/// remove `b'-'` before normalization, you could do something like:
	///
	/// ```
	/// use trimothy::NormalizeWhitespace;
	///
	/// let abnormal: &[u8] = b" Hello -  World!\n";
	/// let normal: Vec<u8> = abnormal.iter()
	///     .filter(|b| b'-'.ne(b))
	///     .copied()
	///     .normalized_whitespace()
	///     .collect();
	/// assert_eq!(normal, b"Hello World!");
	/// ```
	fn normalized_whitespace(self) -> NormalizeWhiteSpaceIter<u8, Copied<Iter<'a, u8>>> {
		self.iter().copied().normalized_whitespace()
	}

	/// # Normalized Control/Whitespace Iterator.
	///
	/// Same as `normalized_whitespace`, but also trim/normalize control
	/// characters.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::NormalizeWhitespace;
	///
	/// let abnormal: &[u8] = b" \0Hello\x1b\0World!\0";
	/// let normal: Vec<u8> = abnormal.normalized_control_and_whitespace().collect();
	/// assert_eq!(normal, b"Hello World!");
	/// ```
	fn normalized_control_and_whitespace(self)
	-> NormalizeWhiteSpaceIter<u8, Copied<Iter<'a, u8>>> {
		self.iter().copied().normalized_control_and_whitespace()
	}
}

impl<'a> NormalizeWhitespace<char, Chars<'a>> for &'a str {
	/// # Normalized Whitespace Iterator.
	///
	/// Return an iterator over the byte/char contents with the edges trimmed,
	/// and all contiguous inner whitespace converted to a single horizontal
	/// space.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::NormalizeWhitespace;
	///
	/// let abnormal: &str = " Hello   World!\n";
	/// let normal: String = abnormal.normalized_whitespace().collect();
	/// assert_eq!(normal, "Hello World!");
	/// ```
	///
	/// This'll work on `char` iterators too. For example, if you wanted to
	/// reverse and normalize a string, you could do something like:
	///
	/// ```
	/// use trimothy::NormalizeWhitespace;
	///
	/// let abnormal: &str = " Hello   World!\n";
	/// let normal: String = abnormal.chars()
	///     .rev()
	///     .normalized_whitespace()
	///     .collect();
	/// assert_eq!(normal, "!dlroW olleH");
	/// ```
	fn normalized_whitespace(self) -> NormalizeWhiteSpaceIter<char, Chars<'a>> {
		self.chars().normalized_whitespace()
	}

	/// # Normalized Control/Whitespace Iterator.
	///
	/// Same as `normalized_whitespace`, but also trim/normalize control
	/// characters.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::NormalizeWhitespace;
	///
	/// let abnormal: &str = " \0Hello\x1b\0World!\0";
	/// let normal: String = abnormal.normalized_control_and_whitespace().collect();
	/// assert_eq!(normal, "Hello World!");
	/// ```
	fn normalized_control_and_whitespace(self)
	-> NormalizeWhiteSpaceIter<char, Chars<'a>> {
		self.chars().normalized_control_and_whitespace()
	}
}



#[derive(Debug)]
/// # (Actual) Normalized Whitespace Iterator.
///
/// This is the actual iterator returned by a
/// `NormalizeWhitespace::normalized_whitespace` implementation.
pub struct NormalizeWhiteSpaceIter<T: Copy + Sized, I: Iterator<Item=T>> {
	iter: I,
	normalize_control: bool,
	next: Option<T>,
}

/// # Implementation Helper
///
/// Implement our custom `NormalizeWhitespace` trait for existing iterators,
/// and implement `Iterator` for the corresponding `NormalizeWhiteSpaceIter`
/// struct.
macro_rules! iter {
	($ty:ty, $is_ws:ident, $is_ctrl:ident, $ws:literal) => (
		impl<I: Iterator<Item=$ty>> NormalizeWhitespace<$ty, I> for I {
			fn normalized_whitespace(mut self) -> NormalizeWhiteSpaceIter<$ty, I> {
				// Return the iterator, starting with the first non-whitespace
				// character.
				let next = self.by_ref().find(|n| ! n.$is_ws());
				NormalizeWhiteSpaceIter {
					iter: self,
					normalize_control: false,
					next,
				}
			}

			fn normalized_control_and_whitespace(mut self)
			-> NormalizeWhiteSpaceIter<$ty, I> {
				// Return the iterator, starting with the first non-whitespace,
				// non-control character.
				let next = self.by_ref().find(|n| ! n.$is_ws() && ! n.$is_ctrl());
				NormalizeWhiteSpaceIter {
					iter: self,
					normalize_control: true,
					next,
				}
			}
		}

		impl<I: Iterator<Item=$ty>> Iterator for NormalizeWhiteSpaceIter<$ty, I> {
			type Item = $ty;

			fn next(&mut self) -> Option<Self::Item> {
				// Anything in the buffer from last time? Return it!
				if let Some(next) = self.next.take() { return Some(next); }

				// Pull the next thing!
				let next = self.iter.next()?;

				// Normalization required.
				if next.$is_ws() || (self.normalize_control && next.$is_ctrl()) {
					// Make sure there's something _after_ this that won't get
					// normalized away, otherwise we've reached the end.
					let ctrl = self.normalize_control;
					self.next = self.by_ref().find(|n| ! n.$is_ws() && (! ctrl || ! n.$is_ctrl()));
					if self.next.is_some() { Some($ws) }
					else { None }
				}
				// It's fine as-is.
				else { Some(next) }
			}

			fn size_hint(&self) -> (usize, Option<usize>) {
				// Because we're potentially dropping things, the lower limit
				// is at most one.
				let lower = usize::from(self.next.is_some());
				let (_, upper) = self.iter.size_hint();
				(lower, upper.map(|n| n + lower))
			}
		}
	);
}

iter!(char, is_whitespace, is_control, ' ');
iter!(u8, is_ascii_whitespace, is_ascii_control, b' ');



#[cfg(test)]
mod test {
	use super::*;
	use alloc::{
		string::String,
		vec::Vec,
	};

	#[test]
	fn t_normalized_control() {
		let example = " \0 Hello\0  Dolly. \x1b";
		// Control is control.
		assert_eq!(
			example.normalized_whitespace().collect::<String>(),
			"\0 Hello\0 Dolly. \x1b",
		);

		// Control is whitespace.
		assert_eq!(
			example.normalized_control_and_whitespace().collect::<String>(),
			"Hello Dolly.",
		);

		let example = example.as_bytes();
		// Control is control.
		assert_eq!(
			example.normalized_whitespace().collect::<Vec<u8>>(),
			b"\0 Hello\0 Dolly. \x1b",
		);

		// Control is whitespace.
		assert_eq!(
			example.normalized_control_and_whitespace().collect::<Vec<u8>>(),
			b"Hello Dolly.",
		);
	}
}
