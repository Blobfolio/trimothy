/*!
# Trimothy: Mutable Trim
*/

use alloc::{
	borrow::Cow,
	boxed::Box,
	string::String,
	vec::Vec,
};
use crate::{
	pattern::MatchPattern,
	TrimSliceMatches,
};



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
/// In keeping with the rest of the library, "whitespace" here means
/// [`char::is_whitespace`] for string sources, and [`u8::is_ascii_whitespace`]
/// for byte sources.
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
/// | `trim_matches_mut` | Trim arbitrary leading and trailing bytes (mutably). |
/// | `trim_start_matches_mut` | Trim arbitrary leading bytes (mutably). |
/// | `trim_end_matches_mut` | Trim arbitrary trailing bytes (mutably). |
///
/// Each of these match methods accept either:
/// * A single T;
/// * An array or slice of T;
/// * A `&BtreeSet<T>`
/// * A custom callback with signature `Fn(T) -> bool`
///
/// Where T is `char` for string sources, and `u8` for byte sources.
///
/// Refer to the individual implementations for examples.
pub trait TrimMatchesMut {
	/// # Matches Type.
	///
	/// This is the "unit" type of the collection, e.g. `char` for `String`,
	/// `u8` for slices, etc.
	type MatchUnit: Copy + Eq + Ord + Sized;

	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// pattern. Refer to the individual implementations for examples.
	fn trim_matches_mut<P: MatchPattern<Self::MatchUnit>>(&mut self, pat: P);

	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading bytes as determined by the provided
	/// pattern. Refer to the individual implementations for examples.
	fn trim_start_matches_mut<P: MatchPattern<Self::MatchUnit>>(&mut self, pat: P);

	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing bytes as determined by the provided
	/// pattern. Refer to the individual implementations for examples.
	fn trim_end_matches_mut<P: MatchPattern<Self::MatchUnit>>(&mut self, pat: P);
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
		self.trim_end_matches_mut(char::is_whitespace);
		self.trim_start_matches_mut(char::is_whitespace);
	}

	#[inline]
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
		self.trim_start_matches_mut(char::is_whitespace);
	}

	#[inline]
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
		self.trim_end_matches_mut(char::is_whitespace);
	}
}

impl TrimMatchesMut for String {
	type MatchUnit = char;

	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing chars as determined by the provided
	/// pattern, which can be:
	/// * A single `char`;
	/// * An array or slice of `char`;
	/// * A `&BTreeSet<char>`;
	/// * A callback with the signature `Fn(char) -> bool`;
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_matches_mut(|c: char| ' ' == c || 'H' == c);
	/// assert_eq!(s, "ello World!");
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_matches_mut([' ', 'H']); // An array works too.
	/// assert_eq!(s, "ello World!");
	/// ```
	fn trim_matches_mut<P: MatchPattern<char>>(&mut self, pat: P) {
		self.trim_end_matches_mut(pat);
		self.trim_start_matches_mut(pat);
	}

	#[inline]
	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading chars as determined by the provided
	/// pattern, which can be:
	/// * A single `char`;
	/// * An array or slice of `char`;
	/// * A `&BTreeSet<char>`;
	/// * A callback with the signature `Fn(char) -> bool`;
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_start_matches_mut(|c: char| ' ' == c || 'H' == c);
	/// assert_eq!(s, "ello World! ");
	///
	/// let mut s = String::from(" Hello World! ");
	/// s.trim_start_matches_mut([' ', 'H']); // An array works too.
	/// assert_eq!(s, "ello World! ");
	/// ```
	fn trim_start_matches_mut<P: MatchPattern<char>>(&mut self, pat: P) {
		if let Some(start) = self.find(#[inline(always)] |c| ! pat.is_match(c)) {
			if start != 0 { self.replace_range(..start, ""); }
		}
		else { self.truncate(0); }
	}

	#[inline]
	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing chars as determined by the provided
	/// pattern, which can be:
	/// * A single `char`;
	/// * An array or slice of `char`;
	/// * A `&BTreeSet<char>`;
	/// * A callback with the signature `Fn(char) -> bool`;
	///
	/// ## Examples
	///
	/// ```
	/// use trimothy::TrimMatchesMut;
	///
	/// let mut s = String::from(" Hello WorlÐ! ");
	/// s.trim_end_matches_mut(|c: char| ' ' == c || '!' == c);
	/// assert_eq!(s, " Hello WorlÐ");
	///
	/// let mut s = String::from(" Hello WorlÐ! ");
	/// s.trim_end_matches_mut([' ', '!']); // An array works too.
	/// assert_eq!(s, " Hello WorlÐ");
	/// ```
	fn trim_end_matches_mut<P: MatchPattern<char>>(&mut self, pat: P) {
		let trimmed_len = self.trim_end_matches(#[inline(always)] |c| pat.is_match(c)).len();
		self.truncate(trimmed_len);
	}
}



impl<'a> TrimMut for Cow<'a, str> {
	#[inline]
	/// # Trim Mut.
	///
	/// Remove leading and trailing whitespace, mutably, preserving the `Cow`
	/// variant.
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimMut;
	///
	/// // Borrowed in, borrowed out.
	/// let mut s: Cow<str> = Cow::Borrowed(" Hello World! ");
	/// s.trim_mut();
	/// assert_eq!(s.as_ref(), "Hello World!");
	/// assert!(matches!(s, Cow::Borrowed(_)));
	///
	/// // Owned in, owned out.
	/// let mut s: Cow<str> = Cow::Owned(String::from(" Hello World! "));
	/// s.trim_mut();
	/// assert_eq!(s.as_ref(), "Hello World!");
	/// assert!(matches!(s, Cow::Owned(_)));
	/// ```
	fn trim_mut(&mut self) {
		match self {
			Cow::Borrowed(s) => { *self = Cow::Borrowed(s.trim()); },
			Cow::Owned(s) => { s.trim_mut(); },
		}
	}

	#[inline]
	/// # Trim Start Mut.
	///
	/// Remove leading whitespace, mutably, preserving the `Cow` variant.
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimMut;
	///
	/// // Borrowed in, borrowed out.
	/// let mut s: Cow<str> = Cow::Borrowed(" Hello World! ");
	/// s.trim_start_mut();
	/// assert_eq!(s.as_ref(), "Hello World! ");
	/// assert!(matches!(s, Cow::Borrowed(_)));
	///
	/// // Owned in, owned out.
	/// let mut s: Cow<str> = Cow::Owned(String::from(" Hello World! "));
	/// s.trim_start_mut();
	/// assert_eq!(s.as_ref(), "Hello World! ");
	/// assert!(matches!(s, Cow::Owned(_)));
	/// ```
	fn trim_start_mut(&mut self) {
		match self {
			Cow::Borrowed(s) => { *self = Cow::Borrowed(s.trim_start()); },
			Cow::Owned(s) => { s.trim_start_mut(); },
		}
	}

	#[inline]
	/// # Trim End Mut.
	///
	/// Remove trailing whitespace, mutably, preserving the `Cow` variant.
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimMut;
	///
	/// // Borrowed in, borrowed out.
	/// let mut s: Cow<str> = Cow::Borrowed(" Hello World! ");
	/// s.trim_end_mut();
	/// assert_eq!(s.as_ref(), " Hello World!");
	/// assert!(matches!(s, Cow::Borrowed(_)));
	///
	/// // Owned in, owned out.
	/// let mut s: Cow<str> = Cow::Owned(String::from(" Hello World! "));
	/// s.trim_end_mut();
	/// assert_eq!(s.as_ref(), " Hello World!");
	/// assert!(matches!(s, Cow::Owned(_)));
	/// ```
	fn trim_end_mut(&mut self) {
		match self {
			Cow::Borrowed(s) => { *self = Cow::Borrowed(s.trim_end()); },
			Cow::Owned(s) => { s.trim_end_mut(); },
		}
	}
}

impl<'a> TrimMatchesMut for Cow<'a, str> {
	type MatchUnit = char;

	#[inline]
	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing chars as determined by the provided
	/// pattern, which can be:
	/// * A single `char`;
	/// * An array or slice of `char`;
	/// * A `&BTreeSet<char>`;
	/// * A callback with the signature `Fn(char) -> bool`;
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimMatchesMut;
	///
	/// // Borrowed in, borrowed out.
	/// let mut s: Cow<str> = Cow::Borrowed(" Hello World! ");
	/// s.trim_matches_mut([' ', 'H']);
	/// assert_eq!(s.as_ref(), "ello World!");
	/// assert!(matches!(s, Cow::Borrowed(_)));
	///
	/// // Owned in, owned out.
	/// let mut s: Cow<str> = Cow::Owned(String::from(" Hello World! "));
	/// s.trim_matches_mut([' ', 'H']);
	/// assert_eq!(s.as_ref(), "ello World!");
	/// assert!(matches!(s, Cow::Owned(_)));
	/// ```
	fn trim_matches_mut<P: MatchPattern<char>>(&mut self, pat: P) {
		match self {
			Cow::Borrowed(s) => {
				*self = Cow::Borrowed(s.trim_matches(#[inline(always)] |c| pat.is_match(c)));
			},
			Cow::Owned(s) => { s.trim_matches_mut(pat); },
		}
	}

	#[inline]
	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading chars as determined by the provided
	/// pattern, which can be:
	/// * A single `char`;
	/// * An array or slice of `char`;
	/// * A `&BTreeSet<char>`;
	/// * A callback with the signature `Fn(char) -> bool`;
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimMatchesMut;
	///
	/// // Borrowed in, borrowed out.
	/// let mut s: Cow<str> = Cow::Borrowed(" Hello World! ");
	/// s.trim_start_matches_mut([' ', 'H']);
	/// assert_eq!(s.as_ref(), "ello World! ");
	/// assert!(matches!(s, Cow::Borrowed(_)));
	///
	/// // Owned in, owned out.
	/// let mut s: Cow<str> = Cow::Owned(String::from(" Hello World! "));
	/// s.trim_start_matches_mut([' ', 'H']);
	/// assert_eq!(s.as_ref(), "ello World! ");
	/// assert!(matches!(s, Cow::Owned(_)));
	/// ```
	fn trim_start_matches_mut<P: MatchPattern<char>>(&mut self, pat: P) {
		match self {
			Cow::Borrowed(s) => {
				*self = Cow::Borrowed(s.trim_start_matches(#[inline(always)] |c| pat.is_match(c)));
			},
			Cow::Owned(s) => { s.trim_start_matches_mut(pat); },
		}
	}

	#[inline]
	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing chars as determined by the provided
	/// pattern, which can be:
	/// * A single `char`;
	/// * An array or slice of `char`;
	/// * A `&BTreeSet<char>`;
	/// * A callback with the signature `Fn(char) -> bool`;
	///
	/// ## Examples
	///
	/// ```
	/// # extern crate alloc;
	/// # use alloc::borrow::Cow;
	/// use trimothy::TrimMatchesMut;
	///
	/// // Borrowed in, borrowed out.
	/// let mut s: Cow<str> = Cow::Borrowed(" Hello World! ");
	/// s.trim_end_matches_mut([' ', '!', 'd', 'l']);
	/// assert_eq!(s.as_ref(), " Hello Wor");
	/// assert!(matches!(s, Cow::Borrowed(_)));
	///
	/// // Owned in, owned out.
	/// let mut s: Cow<str> = Cow::Owned(String::from(" Hello World! "));
	/// s.trim_end_matches_mut([' ', '!', 'd', 'l']);
	/// assert_eq!(s.as_ref(), " Hello Wor");
	/// assert!(matches!(s, Cow::Owned(_)));
	/// ```
	fn trim_end_matches_mut<P: MatchPattern<char>>(&mut self, pat: P) {
		match self {
			Cow::Borrowed(s) => {
				*self = Cow::Borrowed(s.trim_end_matches(#[inline(always)] |c| pat.is_match(c)));
			},
			Cow::Owned(s) => { s.trim_end_matches_mut(pat); },
		}
	}
}



impl TrimMut for Box<[u8]> {
	#[inline]
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
		let trimmed = self.trim_ascii();
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	#[inline]
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
		let trimmed = self.trim_ascii_start();
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	#[inline]
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
		let trimmed = self.trim_ascii_end();
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}
}

impl TrimMatchesMut for Box<[u8]> {
	type MatchUnit = u8;

	#[inline]
	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
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
	fn trim_matches_mut<P: MatchPattern<u8>>(&mut self, pat: P) {
		let trimmed = self.trim_matches(pat);
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	#[inline]
	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
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
	fn trim_start_matches_mut<P: MatchPattern<u8>>(&mut self, pat: P) {
		let trimmed = self.trim_start_matches(pat);
		if trimmed.len() < self.len() { *self = Self::from(trimmed); }
	}

	#[inline]
	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
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
	fn trim_end_matches_mut<P: MatchPattern<u8>>(&mut self, pat: P) {
		let trimmed = self.trim_end_matches(pat);
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
		self.trim_end_mut();
		self.trim_start_mut();
	}

	#[inline]
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
		let slice: &[u8] = self.as_slice();
		let before = slice.len();
		let after = slice.trim_ascii_start().len();
		if after < before {
			if after != 0 { self.copy_within(before - after.., 0); }
			self.truncate(after);
		}
	}

	#[inline]
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
		let trimmed_len = self.trim_ascii_end().len();
		self.truncate(trimmed_len);
	}
}

impl TrimMatchesMut for Vec<u8> {
	type MatchUnit = u8;

	/// # Trim Matches Mut.
	///
	/// Trim arbitrary leading and trailing bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
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
	fn trim_matches_mut<P: MatchPattern<u8>>(&mut self, pat: P) {
		self.trim_end_matches_mut(pat);
		self.trim_start_matches_mut(pat);
	}

	#[inline]
	/// # Trim Start Matches Mut.
	///
	/// Trim arbitrary leading bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
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
	fn trim_start_matches_mut<P: MatchPattern<u8>>(&mut self, pat: P) {
		if let Some(start) = self.iter().copied().position(#[inline(always)] |b| ! pat.is_match(b)) {
			if 0 != start {
				let trimmed_len = self.len() - start;
				self.copy_within(start.., 0);
				self.truncate(trimmed_len);
			}
		}
		else { self.truncate(0); }
	}

	#[inline]
	/// # Trim End Matches Mut.
	///
	/// Trim arbitrary trailing bytes as determined by the provided
	/// pattern, which can be:
	/// * A single `u8`;
	/// * An array or slice of `u8`;
	/// * A `&BTreeSet<u8>`;
	/// * A callback with the signature `Fn(u8) -> bool`;
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
	fn trim_end_matches_mut<P: MatchPattern<u8>>(&mut self, pat: P) {
		let end = self.iter()
			.copied()
			.rposition(#[inline(always)] |b| ! pat.is_match(b))
			.map_or(0, |e| e + 1);
		self.truncate(end);
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn trim_str() {
		use alloc::borrow::ToOwned;

		for v in [
			"ĤéĹlo the WŎrld\u{0300}",
			" ĤéĹlo the WŎrld\u{0300}",
			" \tĤéĹlo the WŎrld\u{0300}",
			"\r \nĤéĹlo\nthe WŎrld\u{0300}",
			" ĤéĹlo the WŎrld\u{0300}\u{2003} ",
			" \tĤéĹlo the WŎrld\u{0300}   ",
			"\r \nĤéĹlo\nthe WŎrld\u{0300} \t\t",
			"ĤéĹlo the WŎrld\u{0300}\0  ",
			"ĤéĹlo the WŎrld\u{0300}\r\r",
			"ĤéĹlo the WŎrld\u{0300} \r\t",
			"\nHello\nWorld\n!\n",
		] {
			let mut v2 = v.to_owned();
			v2.trim_start_mut();
			assert_eq!(v2, v.trim_start());

			v.clone_into(&mut v2);
			v2.trim_end_mut();
			assert_eq!(v2, v.trim_end());

			v.clone_into(&mut v2);
			v2.trim_mut();
			assert_eq!(v2, v.trim());

			v.clone_into(&mut v2);
			v2.trim_matches_mut(|c| c == '\t');
			assert_eq!(v2, v.trim_matches(|c| c == '\t'));

			v.clone_into(&mut v2);
			v2.trim_matches_mut('\t');
			assert_eq!(v2, v.trim_matches(|c| c == '\t'));

			v.clone_into(&mut v2);
			v2.trim_matches_mut(['\t']);
			assert_eq!(v2, v.trim_matches(|c| c == '\t'));

			v.clone_into(&mut v2);
			v2.trim_matches_mut(&['\t']);
			assert_eq!(v2, v.trim_matches(|c| c == '\t'));
		}
	}
}
