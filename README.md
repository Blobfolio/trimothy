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



### TrimSlice

This trait adds the following basic trimming capabilities to `&[u8]`, `Vec<u8>`, and `Box<[u8]>`, similar to those enjoyed by strings.

| Method | Description |
| ------ | ----------- |
| `trim` | Trim leading and trailing (ASCII) whitespace. |
| `trim_start` | Trim leading (ASCII) whitespace. |
| `trim_end` | Trim trailing (ASCII) whitespace. |


### TrimSliceMatches

This trait adds the arbitrary, match-based trimming methods to `&[u8]`, `Vec<u8>`, and `Box<[u8]>`:

| Method | Description |
| ------ | ----------- |
| `trim_matches` | Trim arbitrary leading and trailing bytes via callback. |
| `trim_start_matches` | Trim arbitrary leading bytes via callback. |
| `trim_end_matches` | Trim arbitrary trailing bytes via callback. |


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
| `trim_matches_mut` | Trim arbitrary leading and trailing bytes via callback (mutably). |
| `trim_start_matches_mut` | Trim arbitrary leading bytes via callback (mutably). |
| `trim_end_matches_mut` | Trim arbitrary trailing bytes via callback (mutably). |


### NormalizeWhitespace

This trait exposes an iterator over byte/string slice contents with the edges trimmed, and all contiguous inner whitespace converted to a single horizontal space. This trait is also implemented for existing `u8`/`char` iterators.

| Method | Description |
| ------ | ----------- |
| `normalized_whitespace` | Return said iterator. |



## Installation

The dependency can be added the normal way:

```toml
[dependencies]
trimothy = "0.2"
```



## License

Copyright © 2023 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

This work is free. You can redistribute it and/or modify it under the terms of the Do What The Fuck You Want To Public License, Version 2.

    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    Version 2, December 2004
    
    Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    
    Everyone is permitted to copy and distribute verbatim or modified
    copies of this license document, and changing it is allowed as long
    as the name is changed.
    
    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    
    0. You just DO WHAT THE FUCK YOU WANT TO.
