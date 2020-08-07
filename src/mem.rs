//! Memory utility functions like `memset`, `memcpy`, etc.

#![no_builtins]

use core::{ops::Range, ptr};

mod sealed {
    pub trait Sealed {}
}
/// Trait for machine word types.
///
/// This trait is implemented by unsigned integers representing common machine
/// word sizes.
pub unsafe trait Word: sealed::Sealed + Copy {}

impl sealed::Sealed for u8 {}
impl sealed::Sealed for u16 {}
impl sealed::Sealed for u32 {}
impl sealed::Sealed for u64 {}
impl sealed::Sealed for u128 {}

unsafe impl Word for u8 {}
unsafe impl Word for u16 {}
unsafe impl Word for u32 {}
unsafe impl Word for u64 {}
unsafe impl Word for u128 {}

/// Fills the bytes in the given `range` with the given value `val`.
pub unsafe fn memset<T: Word>(range: Range<*mut T>, val: T) {
    let mut ptr = range.start;
    while ptr < range.end {
        ptr::write_volatile(ptr, val);
        ptr = ptr.offset(1);
    }
}
