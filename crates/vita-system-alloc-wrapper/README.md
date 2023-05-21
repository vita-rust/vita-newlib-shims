# vita-system-alloc-wrapper

[![Crates.io](https://img.shields.io/crates/v/vita-system-alloc-wrapper.svg)](https://crates.io/crates/vita-system-alloc-wrapper)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

The standard library in Rust has a `std::alloc::System` allocator, which internally calls `libc::malloc` and `libc::free` for allocations. For PlayStation Vita, these functions are provided by `newlib`, which uses a [preallocated pool for heap memory](https://github.com/vitasdk/newlib/blob/vita/newlib/libc/sys/vita/sbrk.c).

This should work and works for the majority of cases. Unfortunately though, with the `-O3` optimization level compiled specifically for the PlayStation Vita target, it is possible for LLVM to incorrectly optimize `libc::free` calls, leading to crashes when heap-allocated structs are dropped in Rust.

This crate does a hack to fix that by providing a custom allocator, which proxies, all calls to `std::alloc::System` allocator, and wrapping `dealloc` call in `std::hint::black_box`, which prevents LLVM from doing any optimizations on `libc::free` call.

If you are experiencing crashes in Drop calls in seemingly correct code on PlayStation Vita, try adding this crate as a dependency, and registering provided allocator as a `GlobalAllocator`:


```rust
#[cfg(target_os = "vita")]
#[global_allocator]
static GLOBAL: vita_system_alloc_wrapper::SystemAllocWrapper = vita_system_alloc_wrapper::SystemAllocWrapper;
```