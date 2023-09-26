# vita-newlib-shims

[![Crates.io](https://img.shields.io/crates/v/vita-newlib-shims.svg)](https://crates.io/crates/vita-newlib-shims)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/vita-rust/std#license)

Standard library support for PlayStation Vita in Rust relies on Vita SDK newlib. Unfortunately, [not all](https://github.com/vitasdk/newlib/issues/86) of the POSIX functions are implemented in it, including those that Rust `std` relies on. 

This repository aims to fix that by providing shims for the missing functions.

To use this crate you need to:

1. Add it as a dependency to your project
   ```bash
   cargo add vita-newlib-shims
   ```
2. Use it somewhere in your lib/main, so that the code is actually compiled and linked
   ```rust
   #[cfg(target_os = "vita")]
   use vita_newlib_shims as _;
   ```

## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed at your option under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
