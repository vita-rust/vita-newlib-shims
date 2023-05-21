# vita-newlib-shims

[![Crates.io](https://img.shields.io/crates/v/vita-newlib-shims.svg)](https://crates.io/crates/vita-newlib-shims)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Standard library support for PlayStation Vita in Rust relies on Vita SDK newlib. Unfortunately, [not all](https://github.com/vitasdk/newlib/issues/86) of the POSIX functions are implemented in it, including those that Rust `std` relies on. 

This repository aims to fix that by providing shims for the missing functions.

Keep in mind that:
- These functions are missing from newlib because they can't be implemented or don't make sense on Vita. These shims simply provide signatures for the missing functions with implementation panicking in the runtime. You should not **use** code that relies on these functions.
- If you are using link time optimization, this crate is probably not required, because, you should not use functionality that requires these shims anyway, and the code that uses them will be optimized out by the linker.
- In most cases, you do want to use link time optimization, because you will probably be using only a small subset of std in your application, and the remaining std implementation will take a lot of space in your binary.
