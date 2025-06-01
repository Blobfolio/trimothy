# Trimothy

[![docs.rs](https://img.shields.io/docsrs/trimothy.svg?style=flat-square&label=docs.rs)](https://docs.rs/trimothy/)
[![changelog](https://img.shields.io/crates/v/trimothy.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/trimothy/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/trimothy.svg?style=flat-square&label=crates.io)](https://crates.io/crates/trimothy)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/trimothy/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/trimothy/actions)
[![deps.rs](https://deps.rs/crate/trimothy/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/trimothy/)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/trimothy/issues)

Trimothy is a small library that expands on the limited String- and slice-trimming capabilities provided by the standard library.

If any of these methods happened to be introduced into stable Rust in the future, they will simply be removed from here.

This crate is `#![no_std]`-compatible.



### TrimSliceMatches

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


### TrimMut

This trait brings _mutable_ trimming support to `String`, `Vec<u8>`, and `Box<[u8]>`.

| Method | Description |
| ------ | ----------- |
| `trim_mut` | Trim leading and trailing whitespace (mutably). |
| `trim_start_mut` | Trim leading whitespace (mutably). |
| `trim_end_mut` | Trim trailing whitespace (mutably). |


### TrimMatchesMut

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

Where T is `char` for string sources, and `u8` for byte sources.


### TrimNormal

This trait adds a single `trim_and_normalize` method to owned and borrowed string and byte slices that trims leading/trailing whitespace, and compacts/normalizes spans of _inner_ whitespace to a single horizontal space.

| Method | Description |
| ------ | ----------- |
| `trim_and_normalize` | Trim, normalize, and return. |

The `TrimNormalBytes` and `TrimNormalChars` traits can be used to extend
this same functionality to arbitrary iterators of `u8` and `char`,
respectively.



## Installation

The dependency can be added the normal way:

```toml
[dependencies]
trimothy = "0.8"
```
