# std-newlib - an umbrella repository for running Rust std on PlayStation Vita with newlib

This repository hosts source code for crates intended to improve Rust std support for PlayStation Vita.

## [vita-newlib-shims](./crates/vita-newlib-shims/README.md)

Shims for functions missing from Vita SDK newlib.


## [vita-system-alloc-wrapper](./crates/vita-system-alloc-wrapper/README.md)

A hack around LLVM optimizing `libc::free` calls in System allocator