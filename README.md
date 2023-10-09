# vita-newlib-shims


[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/vita-rust/vita-newlib-shims#license)
[![Crates.io](https://img.shields.io/crates/v/vita-newlib-shims.svg)](https://crates.io/crates/vita-newlib-shims)
[![GitHub Actions Build Status](https://img.shields.io/github/actions/workflow/status/vita-rust/vita-newlib-shims/ci.yml)](https://github.com/vita-rust/vita-newlib-shims/actions/workflows/ci.yml)
[![Current Release](https://img.shields.io/github/release/vita-rust/vita-newlib-shims.svg)](https://github.com/vita-rust/vita-newlib-shims/releases)
[![Main Commits RSS Feed](https://img.shields.io/badge/rss-commits-ffa500?logo=rss)](https://github.com/vita-rust/vita-newlib-shims/commits/main.atom)

Standard library support for PlayStation Vita in Rust relies on Vita SDK newlib. Unfortunately, [not all](https://github.com/vitasdk/newlib/issues/86) of the POSIX functions are implemented in it.

This repository aims to temporarily fix that by providing shims for the missing functions.

To use this crate:

1. Add it as a dependency to your project

   ```bash
   cargo add vita-newlib-shims
   ```
2. Import this crate in the root of your project:

   ```rust
   #[cfg(target_os = "vita")]
   use vita_newlib_shims as _;
   ```
3. `VITASDK` environment variable must be set to your [Vita SDK] location.

During build this crate will check the exported symbols from the `libc.a` object file using `arm-vita-eabi-nm` of your [Vita SDK] installation, and will only provide shims for the missing functions.


## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed at your option under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

[Vita SDK]: https://vitasdk.org
