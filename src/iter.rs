/*!
# Trimothy - Normalized Whitespace Iterator
*/

use core::{
	iter::Iterator,
	slice::Iter,
	str::Chars,
};
use crate::TrimSlice;



/// # Normalized Whitespace Iterator.
///
/// This trait adds a `normalized_whitespace` method to byte and string slices
/// for iterating over their contents with the edges trimmed, and all
/// contiguous inner whitespace converted to a single horizontal space.
pub trait NormalizeWhitespace<T> {
	/// # Normalized Whitespace Iterator.
	///
	/// Return an iterator over the byte/char contents with the edges trimmed,
	/// and all contiguous inner whitespace converted to a single horizontal
	/// space.
	fn normalized_whitespace(&self) -> NormalizedWhitespace<T>;
}

impl<'a> NormalizeWhitespace<Iter<'a, u8>> for &'a [u8] {
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
	fn normalized_whitespace(&self) -> NormalizedWhitespace<Iter<'a, u8>> {
		NormalizedWhitespace {
			iter: self.trim().iter(),
			ws: false,
		}
	}
}

impl<'a> NormalizeWhitespace<Chars<'a>> for &'a str {
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
	fn normalized_whitespace(&self) -> NormalizedWhitespace<Chars<'a>> {
		NormalizedWhitespace {
			iter: self.trim().chars(),
			ws: false,
		}
	}
}



#[derive(Debug)]
/// # (Actual) Normalized Whitespace Iterator.
///
/// This is the actual iterator returned by a
/// `NormalizeWhitespace::normalized_whitespace` implementation.
pub struct NormalizedWhitespace<T> {
	iter: T,
	ws: bool
}

impl<'a> Iterator for NormalizedWhitespace<Iter<'a, u8>> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let next = self.iter.next()?;
			if next.is_ascii_whitespace() {
				if ! self.ws {
					self.ws = true;
					return Some(b' ');
				}
			}
			else {
				self.ws = false;
				return Some(*next);
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let upper = self.iter.len();
		(0, Some(upper))
	}
}

impl<'a> Iterator for NormalizedWhitespace<Chars<'a>> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let next = self.iter.next()?;
			if next.is_whitespace() {
				if ! self.ws {
					self.ws = true;
					return Some(' ');
				}
			}
			else {
				self.ws = false;
				return Some(next);
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let (_, upper) = self.iter.size_hint();
		(0, upper)
	}
}
