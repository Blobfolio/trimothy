/*!
# Trimothy: Match Patterns
*/

use alloc::collections::BTreeSet;



/// # Pattern Trait.
///
/// This trait is used to enable flexible pattern arguments in our
/// match-trimming methods, similar to [`core::str::pattern::Pattern`].
///
/// More specifically, it allows those arguments to accept:
/// * A single T;
/// * An array or slice of T;
/// * A `&BTreeSet<T>`;
/// * A custom callback with signature `Fn(T) -> bool`;
pub trait MatchPattern<T: Copy + Eq + Ord + Sized>: Copy + Sized {
	/// # Is Match?
	///
	/// Returns `true` if `thing` should be trimmed.
	fn is_match(self, thing: T) -> bool;
}



impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for T {
	#[inline]
	/// # Match Self.
	fn is_match(self, thing: T) -> bool { self == thing }
}

impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for &[T] {
	#[inline]
	/// # Match Slice.
	fn is_match(self, thing: T) -> bool { self.contains(&thing) }
}

impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for &BTreeSet<T> {
	#[inline]
	/// # Match Set.
	fn is_match(self, thing: T) -> bool { self.contains(&thing) }
}

impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for [T; 1] {
	#[inline]
	/// # Match Array of One.
	fn is_match(self, thing: T) -> bool { self[0] == thing }
}

impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for &[T; 1] {
	#[inline]
	/// # Match Array of One.
	fn is_match(self, thing: T) -> bool { self[0] == thing }
}

impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for [T; 2] {
	#[inline]
	/// # Match Array of Two.
	fn is_match(self, thing: T) -> bool { self[0] == thing || self[1] == thing }
}

impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for &[T; 2] {
	#[inline]
	/// # Match Array of Two.
	fn is_match(self, thing: T) -> bool { self[0] == thing || self[1] == thing }
}



// Note: for some reason Rust things FN(T) conflicts with T, so we have to be
// specific. Haha.

impl<F: Fn(u8) -> bool + Copy> MatchPattern<u8> for F {
	#[inline]
	/// # Custom Match.
	fn is_match(self, thing: u8) -> bool { self(thing) }
}

impl<F: Fn(char) -> bool + Copy> MatchPattern<char> for F {
	#[inline]
	/// # Custom Match.
	fn is_match(self, thing: char) -> bool { self(thing) }
}



/// # Helper: 3+ Array Implementations.
macro_rules! arr {
	($($size:literal),+ $(,)?) => ($(
		impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for [T; $size] {
			#[inline]
			/// # Array Match.
			fn is_match(self, thing: T) -> bool { self.contains(&thing) }
		}
		impl<T: Copy + Eq + Ord + Sized> MatchPattern<T> for &[T; $size] {
			#[inline]
			/// # Array Match.
			fn is_match(self, thing: T) -> bool { self.contains(&thing) }
		}
	)+);
}

arr!(
	         3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16,
	17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
);



#[cfg(test)]
mod test {
	use super::*;

	/// # Strip Method.
	const fn strip_b(b: u8) -> bool { b == b'b' }

	#[test]
	fn t_patterns() {
		// Single.
		assert!(b'b'.is_match(b'b'));
		assert!(! b'b'.is_match(b'.'));

		// Array.
		let arr: [u8; 1] = [b'b'];
		assert!(arr.is_match(b'b'));
		assert!(! arr.is_match(b'a'));

		let arr: [u8; 2] = [b'b', b'.'];
		assert!(arr.is_match(b'b'));
		assert!(arr.is_match(b'.'));
		assert!(! arr.is_match(b'a'));

		let arr: [u8; 3] = [b'b', b'.', b'!'];
		assert!(arr.is_match(b'b'));
		assert!(arr.is_match(b'.'));
		assert!(arr.is_match(b'!'));
		assert!(! arr.is_match(b'a'));

		// Slice.
		assert!(arr.as_slice().is_match(b'b'));
		assert!(arr.as_slice().is_match(b'.'));
		assert!(arr.as_slice().is_match(b'!'));
		assert!(! arr.as_slice().is_match(b'a'));

		// BTreeSet.
		let set = BTreeSet::from(arr);
		assert!(set.is_match(b'b'));
		assert!(set.is_match(b'.'));
		assert!(set.is_match(b'!'));
		assert!(! set.is_match(b'a'));

		// Method.
		assert!(strip_b.is_match(b'b'));
		assert!(! strip_b.is_match(b'B'));

		// Closure.
		let foo = |b: u8| -> bool { b == b'b' };
		assert!(foo.is_match(b'b'));
		assert!(! foo.is_match(b'X'));
	}
}
