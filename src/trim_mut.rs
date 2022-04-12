/*!
# Trimothy - Mutable Trim
*/

#[cfg(not(feature = "std"))]
use alloc::{
	boxed::Box,
	string::String,
	vec::Vec,
};

use crate::{
	not_whitespace,
	TrimSlice,
	TrimSliceMatches,
};

#[cfg(feature = "std")]
use std::ptr::copy;

#[cfg(not(feature = "std"))]
use core::intrinsics::copy;



/// # Mutable Trim.
///
/// The [`TrimMut`] trait exposes mutable trimming methods for `String`,
/// `Vec<u8>`, and `Box<[u8]>`.
///
/// The trait methods included are:
///
/// | Method | Description |
/// | ------ | ----------- |
/// | `trim_mut` | Trim leading and trailing whitespace (mutably). |
/// | `trim_start_mut` | Trim leading whitespace (mutably). |
/// | `trim_end_mut` | Trim trailing whitespace (mutably). |
///
/// **Note:** The behaviors of these methods are consistent with their immutable
/// counterparts, meaning that Strings will trim [`char::is_whitespace`], while
/// slices will only trim [`u8::is_ascii_whitespace`].
///
/// Refer to the individual implementations for examples.
pub trait TrimMut {
	/// # Trim Mut.
	///
	/// Remove leading and trailing whitespace, mutably. Refer to the
	/// individual implementations for examples.
	fn trim_mut(&mut self);

	/// # Trim Start Mut.
	///
	/// Remove leading whitespace, mutably. Refer to the individual
	/// implementations for examples.
	fn trim_start_mut(&mut self);

	/// # Trim End Mut.
	///
	/// Remove trailing whitespace, mutably. Refer to the individual
	/// implementations for examples.
	fn trim_end_mut(&mut self);
}



/// # Mutable Trim (Matches).
///
/// The [`TrimMatchesMut`] trait exposes mutable match-based trimming methods for
/// `String`, `Vec<u8>`, and `Box<[u8]>`.
///
/// The trait methods included are:
///
/// | Method | Description |
/// | ------ | ----------- |
/// | `trim_matches_mut` | Trim arbitrary leading and trailing bytes via callback (mutably). |
/// | `trim_start_matches_mut` | Trim arbitrary leading bytes via callback (mutably). |
/// | `trim_end_matches_mut` | Trim arbitrary trailing bytes via callback (mutably). |
///
/// **Note:** To maintain consistency with their immutable counterparts, the
/// `String` implementation expects callbacks that match a `char`, while the
/// `Vec<u8>` and `Box<[u8]>` implementations expect callbacks that match a `u8`.
///
/// Refer to the individual implementations for examples.
pub trait TrimMatchesMut {
	/// # Matches Type.
	///
	/// This is the "unit" type of the collection, e.g. `char` for `String`,
	/// `u8` for slices, etc.
	type MatchUnit;

	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// callback, where a return value of `true` means trim. Refer to the
	/// individual implementations for examples.
	fn trim_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool;

	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	fn trim_start_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool;

	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	fn trim_end_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool;
}



impl TrimMut for String {
	/// # Trim Mut.
	///
	/// Remove leading and trailing whitespace, mutably.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_mut();
	/// assert_eq!(s, "Hello World!");
	/// ```
	fn trim_mut(&mut self) {
		let trimmed = self.trim();
		let trimmed_len = trimmed.len();

		if trimmed_len < self.len() {
			if 0 < trimmed_len {
				let trimmed_ptr = trimmed.as_ptr();

				// Safety: we're just moving the trimmed portion to the start
				// of the buffer and chopping the length to match.
				unsafe {
					let v = self.as_mut_vec();
					copy(trimmed_ptr, v.as_mut_ptr(), trimmed_len);
					v.set_len(trimmed_len);
				}
			}
			else { self.truncate(0); }
		}
	}

	/// # Trim Start Mut.
	///
	/// Remove leading whitespace, mutably.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_start_mut();
	/// assert_eq!(s, "Hello World! ");
	/// ```
	fn trim_start_mut(&mut self) {
		let trimmed = self.trim_start();
		let trimmed_len = trimmed.len();

		if trimmed_len < self.len() {
			if 0 < trimmed_len {
				let trimmed_ptr = trimmed.as_ptr();

				// Safety: we're just moving the trimmed portion to the start
				// of the buffer and chopping the length to match.
				unsafe {
					let v = self.as_mut_vec();
					copy(trimmed_ptr, v.as_mut_ptr(), trimmed_len);
					v.set_len(trimmed_len);
				}
			}
			else { self.truncate(0); }
		}
	}

	/// # Trim End Mut.
	///
	/// Remove trailing whitespace, mutably.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_end_mut();
	/// assert_eq!(s, " Hello World!");
	/// ```
	fn trim_end_mut(&mut self) {
		let trimmed = self.trim_end();
		let trimmed_len = trimmed.len();
		self.truncate(trimmed_len);
	}
}

impl TrimMatchesMut for String {
	type MatchUnit = char;

	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// callback, where a return value of `true` means trim. Refer to the
	/// individual implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_matches_mut(|c: char| ' ' == c || 'H' == c);
	/// assert_eq!(s, "ello World!");
	/// ```
	fn trim_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		let trimmed = self.trim_matches(cb);
		let trimmed_len = trimmed.len();

		if trimmed_len < self.len() {
			if 0 < trimmed_len {
				let trimmed_ptr = trimmed.as_ptr();

				// Safety: we're just moving the trimmed portion to the start
				// of the buffer and chopping the length to match.
				unsafe {
					let v = self.as_mut_vec();
					copy(trimmed_ptr, v.as_mut_ptr(), trimmed_len);
					v.set_len(trimmed_len);
				}
			}
			else { self.truncate(0); }
		}
	}

	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_start_matches_mut(|c: char| ' ' == c || 'H' == c);
	/// assert_eq!(s, "ello World! ");
	/// ```
	fn trim_start_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		let trimmed = self.trim_start_matches(cb);
		let trimmed_len = trimmed.len();

		if trimmed_len < self.len() {
			if 0 < trimmed_len {
				let trimmed_ptr = trimmed.as_ptr();

				// Safety: we're just moving the trimmed portion to the start
				// of the buffer and chopping the length to match.
				unsafe {
					let v = self.as_mut_vec();
					copy(trimmed_ptr, v.as_mut_ptr(), trimmed_len);
					v.set_len(trimmed_len);
				}
			}
			else { self.truncate(0); }
		}
	}

	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_end_matches_mut(|c: char| ' ' == c || '!' == c);
	/// assert_eq!(s, " Hello World");
	/// ```
	fn trim_end_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		let trimmed = self.trim_end_matches(cb);
		let trimmed_len = trimmed.len();
		self.truncate(trimmed_len);
	}
}



impl TrimMut for Box<[u8]> {
	/// # Trim Mut.
	///
	/// Remove leading and trailing (ASCII) whitespace, replacing `Self` with
	/// a new boxed slice if necessary.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut v = Box::<[u8]>::from(&b" Hello World! "[..]);
	/// v.trim_mut();
	/// assert_eq!(v, Box::from(&b"Hello World!"[..]));
	/// ```
	fn trim_mut(&mut self) {
		let trimmed = self.trim();
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	/// # Trim Start Mut.
	///
	/// Remove leading (ASCII) whitespace, replacing `Self` with a new boxed
	/// slice if necessary.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut v = Box::<[u8]>::from(&b" Hello World! "[..]);
	/// v.trim_start_mut();
	/// assert_eq!(v, Box::from(&b"Hello World! "[..]));
	/// ```
	fn trim_start_mut(&mut self) {
		let trimmed = self.trim_start();
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	/// # Trim End Mut.
	///
	/// Remove trailing (ASCII) whitespace, replacing `Self` with a new boxed
	/// slice if necessary.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut v = Box::<[u8]>::from(&b" Hello World! "[..]);
	/// v.trim_end_mut();
	/// assert_eq!(v, Box::from(&b" Hello World!"[..]));
	/// ```
	fn trim_end_mut(&mut self) {
		let trimmed = self.trim_end();
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}
}

impl TrimMatchesMut for Box<[u8]> {
	type MatchUnit = u8;

	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// callback, where a return value of `true` means trim. Refer to the
	/// individual implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut v = Box::<[u8]>::from(&b" Hello World! "[..]);
	/// v.trim_matches_mut(|b: u8| b'!' == b || b.is_ascii_whitespace());
	/// assert_eq!(v, Box::from(&b"Hello World"[..]));
	/// ```
	fn trim_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		let trimmed = self.trim_matches(cb);
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut v = Box::<[u8]>::from(&b" Hello World! "[..]);
	/// v.trim_start_matches_mut(|b: u8| b'!' == b || b.is_ascii_whitespace());
	/// assert_eq!(v, Box::from(&b"Hello World! "[..]));
	/// ```
	fn trim_start_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		let trimmed = self.trim_start_matches(cb);
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut v = Box::<[u8]>::from(&b" Hello World! "[..]);
	/// v.trim_end_matches_mut(|b: u8| b'!' == b || b.is_ascii_whitespace());
	/// assert_eq!(v, Box::from(&b" Hello World"[..]));
	/// ```
	fn trim_end_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		let trimmed = self.trim_end_matches(cb);
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}
}



impl TrimMut for Vec<u8> {
	/// # Trim Mut.
	///
	/// Remove leading and trailing (ASCII) whitespace, mutably.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut v = b" Hello World! ".to_vec();
	/// v.trim_mut();
	/// assert_eq!(v, b"Hello World!");
	/// ```
	fn trim_mut(&mut self) {
		self.trim_start_mut();
		self.trim_end_mut();
	}

	/// # Trim Start Mut.
	///
	/// Remove leading (ASCII) whitespace, mutably.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut v = b" Hello World! ".to_vec();
	/// v.trim_start_mut();
	/// assert_eq!(v, b"Hello World! ");
	/// ```
	fn trim_start_mut(&mut self) {
		if let Some(start) = self.iter().position(not_whitespace) {
			if 0 < start {
				let trimmed_len = self.len() - start;

				// Safety: we're just moving the trimmed portion to the start
				// of the buffer and chopping the length to match.
				unsafe {
					copy(self.as_ptr().add(start), self.as_mut_ptr(), trimmed_len);
					self.set_len(trimmed_len);
				}
			}
		}
		else { self.truncate(0); }
	}

	/// # Trim End Mut.
	///
	/// Remove trailing (ASCII) whitespace, mutably.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMut;
	///
	/// let mut v = b" Hello World! ".to_vec();
	/// v.trim_end_mut();
	/// assert_eq!(v, b" Hello World!");
	/// ```
	fn trim_end_mut(&mut self) {
		if let Some(end) = self.iter().rposition(not_whitespace) {
			self.truncate(end + 1);
		}
		else { self.truncate(0); }
	}
}

impl TrimMatchesMut for Vec<u8> {
	type MatchUnit = u8;

	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// callback, where a return value of `true` means trim. Refer to the
	/// individual implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut v = b" Hello World! ".to_vec();
	/// v.trim_matches_mut(|b: u8| b.is_ascii_whitespace() || b.is_ascii_uppercase());
	/// assert_eq!(v, b"ello World!");
	/// ```
	fn trim_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		let trimmed = self.trim_matches(cb);
		let trimmed_len = trimmed.len();

		if trimmed_len < self.len() {
			if 0 < trimmed_len {
				let trimmed_ptr = trimmed.as_ptr();

				// Safety: we're just moving the trimmed portion to the start
				// of the buffer and chopping the length to match.
				unsafe {
					copy(trimmed_ptr, self.as_mut_ptr(), trimmed_len);
					self.set_len(trimmed_len);
				}
			}
			else { self.truncate(0); }
		}
	}

	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut v = b" Hello World! ".to_vec();
	/// v.trim_start_matches_mut(|b: u8| b.is_ascii_whitespace() || b.is_ascii_uppercase());
	/// assert_eq!(v, b"ello World! ");
	/// ```
	fn trim_start_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		if let Some(start) = self.iter().position(|b: &u8| ! cb(*b)) {
			if 0 < start {
				let trimmed_len = self.len() - start;

				// Safety: we're just moving the trimmed portion to the start
				// of the buffer and chopping the length to match.
				unsafe {
					copy(self.as_ptr().add(start), self.as_mut_ptr(), trimmed_len);
					self.set_len(trimmed_len);
				}
			}
		}
		else { self.truncate(0); }
	}

	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing bytes as determined by the provided callback,
	/// where a return value of `true` means trim. Refer to the individual
	/// implementations for examples.
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut v = b" Hello World! ".to_vec();
	/// v.trim_end_matches_mut(|b: u8| b.is_ascii_whitespace() || b.is_ascii_uppercase());
	/// assert_eq!(v, b" Hello World!");
	/// ```
	fn trim_end_matches_mut<F>(&mut self, cb: F)
	where F: Fn(Self::MatchUnit) -> bool {
		if let Some(end) = self.iter().rposition(|b: &u8| ! cb(*b)) {
			self.truncate(end + 1);
		}
		else { self.truncate(0); }
	}
}
