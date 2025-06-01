# Changelog



## [0.8.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.8.0) - 2025-06-01

### Changed

* Bump `brunch` to `0.10` (dev)
* Bump MSRV to `1.87`
* Fix deps.rs badge (docs)



## [0.7.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.7.0) - 2025-02-25

### Changed

* Bump `brunch` to `0.9` (dev)
* Bump MSRV to `1.85`
* Bump Rust edition to `2024`
* Miscellaneous code cleanup and lints



## [0.6.1](https://github.com/Blobfolio/trimothy/releases/tag/v0.6.1) - 2025-01-09

### Changed

* Bump `brunch` to `0.8` (dev)
* Miscellaneous code cleanup and lints



## [0.6.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.6.0) - 2024-11-28

### New

* impl `TrimMut` for `Cow<str>`
* impl `TrimMatchesMut` for `Cow<str>`
* impl `TrimMut` for `Cow<[u8]>`
* impl `TrimMatchesMut` for `Cow<[u8]>`

### Changed

* Bump `brunch` to `0.7` (dev)
* Miscellaneous code cleanup and lints



## [0.5.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.5.0) - 2024-11-19

### New

* `TrimNormal` trait
* `TrimNormalBytes` trait
* `TrimNormalChars` trait

### Removed

* `NormalizeWhitespace` (use a `TrimNormal` trait instead)



## [0.4.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.4.0) - 2024-11-15

### Changed

* `trim_mut`, `trim_start_mut`, and `trim_end_mut` have been generalized to accept any of the following:
  * A single T;
  * An array or slice of T;
  * A `&BTreeSet<T>`;
  * A custom callback with signature `Fn(T) -> bool`;
    * Callbacks now require `Copy`;
* Bump MSRV to `1.81`



## [0.3.1](https://github.com/Blobfolio/trimothy/releases/tag/v0.3.1) - 2024-09-05

### Changed

* Miscellaneous code cleanup and lints
* Bump `brunch` to `0.6`



## [0.3.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.3.0) - 2024-07-29

### Changed

* Bump MSRV `1.80.0`
* Minor code tweaks

### Removed

* `TrimSlice` trait



## [0.2.3](https://github.com/Blobfolio/trimothy/releases/tag/v0.2.3) - 2024-07-25

### Deprecated

* `TrimSlice` trait; prefer native slice `trim_ascii` methods now that they're stable



## [0.2.2](https://github.com/Blobfolio/trimothy/releases/tag/v0.2.2) - 2023-10-04

### New

* `NormalizeWhitespace::normalized_control_and_whitespace` member method



## [0.2.1](https://github.com/Blobfolio/trimothy/releases/tag/v0.2.1) - 2023-10-04

### Changed

* Refactor/extend `NormalizeWhitespace` to work for all `u8`/`char` `Iterator`s



## [0.2.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.2.0) - 2023-10-03

### New

* `NormalizeWhitespace` trait



## [0.1.8](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.8) - 2023-06-01

### Changed

* Fix UTF8 issue with String::trim_end*



## ~~0.1.7~~ - 2023-06-01

### Changed

* Remove all `unsafe` blocks
* Improve CI coverage



## [0.1.6](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.6) - 2023-01-26

### Changed

* Bump brunch `0.4` (dev)



## [0.1.5](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.5) - 2022-12-27

### Changed

* Minor slice trim performance improvements
* Update ci badge syntax (docs)



## [0.1.4](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.4) - 2022-09-22

### Changed

* Lower MSRV `1.56`
* Improve docs



## [0.1.3](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.3) - 2022-05-30

### Changed

* Minor performance improvements for `TrimSlice` implementations



## [0.1.2](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.2) - 2022-04-30

### Changed

* Make crate `#![no_std]` w/o any feature gates



## [0.1.1](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.1) - 2022-04-11

### Changed

* Minor performance improvements

### Fixed

* Return empty slice when all bytes match trim predicate.
* Markdown (docs) formatting issues.



## [0.1.0](https://github.com/Blobfolio/trimothy/releases/tag/v0.1.0) - 2022-04-11

Initial release!
