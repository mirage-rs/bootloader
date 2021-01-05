//! Utility functions assisting with memory manipulation.
//!
//! Core Requirements for the Rust std are also defined here.

use core::{mem, ops::Range, ptr};

/// Fills the block of memory denoted by the given `range` with `value`.
pub unsafe fn memset(range: Range<*mut u32>, val: u32) {
    let mut ptr = range.start;
    while ptr < range.end {
        // Utilize write_volatile to prevent this from being transformed into a `memclr`.
        ptr::write_volatile(ptr, val);
        ptr = ptr.offset(1);
    }
}

/// Clears the block of memory denoted by the given `range` by filling it with zeroes.
pub unsafe fn clear_mem(range: Range<*mut u32>) {
    memset(range, mem::zeroed());
}

/// Rust version of the libc `memcmp` function.
///
/// Rust needs some core requirements, which `memcmp` is one of them and currently
/// the only one that is required.
#[no_mangle]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut idx = 0;

    while idx < n {
        let a = *a.add(idx);
        let b = *b.add(idx);
        if a != b {
            return a as i32 - b as i32;
        }
        idx += 1;
    }

    0
}
