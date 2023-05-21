# std-newlib

[![Rust](https://github.com/vita-rust/std-newlib/workflows/check/badge.svg)](https://github.com/vita-rust/std-newlib/actions)

This repository hosts source code for crates intended to improve Rust std support for PlayStation Vita.

## [vita-newlib-shims](./crates/vita-newlib-shims/README.md)

[![Crates.io](https://img.shields.io/crates/v/vita-newlib-shims.svg)](https://crates.io/crates/vita-newlib-shims)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Shims for functions missing from Vita SDK newlib.


## [vita-system-alloc-wrapper](./crates/vita-system-alloc-wrapper/README.md)

[![Crates.io](https://img.shields.io/crates/v/vita-system-alloc-wrapper.svg)](https://crates.io/crates/vita-system-alloc-wrapper)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A hack around LLVM optimizing `libc::free` calls in System allocator