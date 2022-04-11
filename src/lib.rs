/*!
# Trimothy

Trimothy is a small library that expands on the limited String- and slice-trimming capabilities provided by the standard library.

If any of these methods happened to be introduced into `stable` rust, they will be removed from here.



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
| `trim_mut` | Trim leading and trailing (ASCII) whitespace. |
| `trim_start_mut` | Trim leading (ASCII) whitespace. |
| `trim_end_mut` | Trim trailing (ASCII) whitespace. |


### [`TrimMatchesMut`]

This trait brings _mutable_ match-based trimming `String`, `Vec<u8>`, and `Box<[u8]>`.

| Method | Description |
| ------ | ----------- |
| `trim_matches_mut` | Trim arbitrary leading and trailing bytes via callback. |
| `trim_start_matches_mut` | Trim arbitrary leading bytes via callback. |
| `trim_end_matches_mut` | Trim arbitrary trailing bytes via callback. |



## Installation

The dependency can be added the normal way:

```ignore,toml
[dependencies]
trimothy = "0.1"
```

To use this library with `no_std` environments — albeith with `alloc` — disable the default `std` crate feature:

```ignore,toml
[dependencies.trimothy]
version = "0.1"
default-features = false
*/

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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
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
