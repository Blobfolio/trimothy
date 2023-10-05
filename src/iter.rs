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
/// This can be called on an `&[u8]` or `&str` directly, or any iterator
/// yielding owned `u8` or `char` items.
pub trait NormalizeWhitespace<T: Copy + Sized, I: Iterator<Item=T>> {
	/// # Normalized Whitespace Iterator.
	///
	/// Modify a byte or char iterator to trim the ends, and convert all
	/// contiguous inner whitespace to a single horizontal space.
	fn normalized_whitespace(self) -> NormalizeWhiteSpaceIter<T, I>;
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
}



#[derive(Debug)]
/// # (Actual) Normalized Whitespace Iterator.
///
/// This is the actual iterator returned by a
/// `NormalizeWhitespace::normalized_whitespace` implementation.
pub struct NormalizeWhiteSpaceIter<T: Copy + Sized, I: Iterator<Item=T>> {
	iter: I,
	next: Option<T>,
}

/// # Helper: Implementations
///
/// Implement our custom `NormalizeWhitespace` trait for existing iterators,
/// and implement `Iterator` for the corresponding `NormalizeWhiteSpaceIter`
/// struct.
macro_rules! iter {
	($ty:ty, $is:ident, $ws:literal) => (
		impl<I: Iterator<Item=$ty>> NormalizeWhitespace<$ty, I> for I {
			fn normalized_whitespace(mut self) -> NormalizeWhiteSpaceIter<$ty, I> {
				// Return the iterator, starting with the first non-whitespace
				// character.
				let next = self.by_ref().find(|n| ! n.$is());
				NormalizeWhiteSpaceIter {
					iter: self,
					next,
				}
			}
		}

		impl<I: Iterator<Item=$ty>> Iterator for NormalizeWhiteSpaceIter<$ty, I> {
			type Item = $ty;

			fn next(&mut self) -> Option<Self::Item> {
				// Anything in the buffer?
				if let Some(next) = self.next.take() { return Some(next); }

				// Pull the next thing.
				let next = self.iter.next()?;
				if next.$is() {
					// If there's something other than whitespace later on, return a
					// single horizontal space. Otherwise we're done.
					self.next = self.by_ref().find(|n| ! n.$is());
					if self.next.is_some() { Some($ws) }
					else { None }
				}
				// Passthrough any non-whitespace bits.
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

iter!(char, is_whitespace, ' ');
iter!(u8, is_ascii_whitespace, b' ');
