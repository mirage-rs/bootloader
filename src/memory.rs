//! Utility functions assisting with memory manipulation.

#![no_builtins]

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
