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



### [`TrimSlice`]

This trait adds the following basic trimming capabilities to `&[u8]`, `Vec<u8>`, and `Box<[u8]>`, similar to those enjoyed by strings.

| Method | Description |
| ------ | ----------- |
| `trim` | Trim leading and trailing (ASCII) whitespace. |
| `trim_start` | Trim leading (ASCII) whitespace. |
| `trim_end` | Trim trailing (ASCII) whitespace. |


### [`TrimSliceMatches`]

This trait adds the arbitrary, match-based trimming methods to `&[u8]`, `Vec<u8>`, and `Box<[u8]>`:

| Method | Description |
| ------ | ----------- |
| `trim_matches` | Trim arbitrary leading and trailing bytes via callback. |
| `trim_start_matches` | Trim arbitrary leading bytes via callback. |
| `trim_end_matches` | Trim arbitrary trailing bytes via callback. |


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
| `trim_matches_mut` | Trim arbitrary leading and trailing bytes via callback (mutably). |
| `trim_start_matches_mut` | Trim arbitrary leading bytes via callback (mutably). |
| `trim_end_matches_mut` | Trim arbitrary trailing bytes via callback (mutably). |
*/

#![forbid(unsafe_code)]

#![warn(
	clippy::filetype_is_file,
	clippy::integer_division,
	clippy::needless_borrow,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::suboptimal_flops,
	clippy::unneeded_field_pattern,
	macro_use_extern_crate,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]
#![allow(clippy::module_name_repetitions)]

#![no_std]

extern crate alloc;

mod trim_mut;
mod trim_slice;

pub use trim_mut::{
	TrimMut,
	TrimMatchesMut,
};
pub use trim_slice::{
	TrimSlice,
	TrimSliceMatches,
};



#[allow(clippy::trivially_copy_pass_by_ref)] // It's the signature iterator wants.
#[inline]
/// # Not Whitespace.
///
/// This callback is used to find the first or last non-whitespace byte in a
/// slice. It is only split off into its own method to enforce consistency.
pub(crate) const fn not_whitespace(b: &u8) -> bool { ! b.is_ascii_whitespace() }
