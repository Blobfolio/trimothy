/*!
# Trimothy

[![docs.rs](https://img.shields.io/docsrs/trimothy.svg?style=flat-square&label=docs.rs)](https://docs.rs/trimothy/)
[![changelog](https://img.shields.io/crates/v/trimothy.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/trimothy/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/trimothy.svg?style=flat-square&label=crates.io)](https://crates.io/crates/trimothy)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/trimothy/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/trimothy/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/trimothy/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/trimothy)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/trimothy/issues)

Trimothy is a small library that expands on the limited String- and slice-trimming capabilities provided by the standard library.

If any of these methods happened to be introduced into stable Rust in the future, they will simply be removed from here.

This crate is `#![no_std]`-compatible.


### [`TrimSliceMatches`]

This trait adds the arbitrary, match-based trimming methods to `&[u8]`, `Vec<u8>`, and `Box<[u8]>`:

| Method | Description |
| ------ | ----------- |
| `trim_matches` | Trim arbitrary leading and trailing bytes. |
| `trim_start_matches` | Trim arbitrary leading bytes. |
| `trim_end_matches` | Trim arbitrary trailing bytes. |

Each of these match methods accept either:
* A single `u8`;
* An array or slice of `u8`;
* A `&BtreeSet<u8>`
* A custom callback with signature `Fn(u8) -> bool`


### [`TrimMut`]

This trait brings _mutable_ trimming support to `String`, `Vec<u8>`, and `Box<[u8]>`.

| Method | Description |
| ------ | ----------- |
| `trim_mut` | Trim leading and trailing whitespace (mutably). |
| `trim_start_mut` | Trim leading whitespace (mutably). |
| `trim_end_mut` | Trim trailing whitespace (mutably). |


### [`TrimMatchesMut`]

This trait brings _mutable_ match-based trimming `String`, `Vec<u8>`, and `Box<[u8]>`.

| Method | Description |
| ------ | ----------- |
| `trim_matches_mut` | Trim arbitrary leading and trailing bytes (mutably). |
| `trim_start_matches_mut` | Trim arbitrary leading bytes (mutably). |
| `trim_end_matches_mut` | Trim arbitrary trailing bytes (mutably). |

Each of these match methods accept either:
* A single T;
* An array or slice of T;
* A `&BtreeSet<T>`
* A custom callback with signature `Fn(T) -> bool`

Where T is `char` for `String`, and `u8` for `Vec<u8>`/`Box<[u8]>`.



### [`NormalizeWhitespace`]

This trait exposes an iterator over byte/string slice contents that trims the edges and compacts/converts all inner, contiguous spans of whitespace to a single horizontal space.

This trait is implemented for `&[u8]`, `&str`, and `Iterator`s with `u8`/`char` items.

| Method | Description |
| ------ | ----------- |
| `normalized_whitespace` | Return a whitespace-normalizing iterator. |
| `normalized_control_and_whitespace` | Return a control- and whitespace-normalizing iterator. |
*/

#![forbid(unsafe_code)]

#![deny(
	clippy::allow_attributes_without_reason,
	clippy::correctness,
	unreachable_pub,
)]

#![warn(
	clippy::complexity,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::style,

	clippy::allow_attributes,
	clippy::clone_on_ref_ptr,
	clippy::create_dir,
	clippy::filetype_is_file,
	clippy::format_push_string,
	clippy::get_unwrap,
	clippy::impl_trait_in_params,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::todo,
	clippy::undocumented_unsafe_blocks,
	clippy::unneeded_field_pattern,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_in_result,

	macro_use_extern_crate,
	missing_copy_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]

#![expect(clippy::module_name_repetitions, reason = "Repetition is preferred.")]

#![no_std]

extern crate alloc;

mod iter;
mod pattern;
mod trim_mut;
mod trim_slice;

pub use iter::NormalizeWhitespace;
pub use trim_mut::{
	TrimMut,
	TrimMatchesMut,
};
pub use trim_slice::TrimSliceMatches;



#[expect(clippy::trivially_copy_pass_by_ref, reason = "This signature is required.")]
#[inline]
/// # Not Whitespace.
///
/// This callback is used to find the first or last non-whitespace byte in a
/// slice. It is only split off into its own method to enforce consistency.
pub(crate) const fn not_whitespace(b: &u8) -> bool { ! b.is_ascii_whitespace() }
