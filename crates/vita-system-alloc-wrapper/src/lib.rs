use std::alloc::{GlobalAlloc, System};

pub struct SystemAllocWrapper;

unsafe impl GlobalAlloc for SystemAllocWrapper {
    #[inline]
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        System::alloc(&System, layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        System::alloc_zeroed(&System, layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        System::realloc(&System, ptr, layout, new_size)
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: std::alloc::Layout) {
        std::hint::black_box(libc::free(ptr as *mut libc::c_void));
    }
}
