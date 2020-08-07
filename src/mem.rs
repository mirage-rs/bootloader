//! Memory utility functions like `memset`, `memcpy`, etc.

#![no_builtins]

use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, val: u8, n: usize) -> *mut u8 {
    let mut offset = 0;
    while offset < n {
        ptr::write_volatile(ptr.offset(offset as isize), val);
        offset += 1;
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(ptr1: *const u8, ptr2: *const u8, n: usize) -> i32 {
    let mut offset = 0;
    while offset < n {
        let a = ptr::read_volatile(ptr1.offset(offset as isize));
        let b = ptr::read_volatile(ptr2.offset(offset as isize));
        if a != b {
            return a as i32 - b as i32;
        }
        offset += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut offset = 0;
    while offset < n {
        let val = ptr::read_volatile(src.offset(offset as isize));
        ptr::write_volatile(dst.offset(offset as isize), val);
        offset += 1;
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dst as *const u8 {
        let mut offset = n;
        while offset != 0 {
            offset -= 1;
            let val = ptr::read_volatile(src.offset(offset as isize));
            ptr::write_volatile(dst.offset(offset as isize), val);
        }
    } else {
        let mut offset = 0;
        while offset < n {
            let val = ptr::read_volatile(src.offset(offset as isize));
            ptr::write_volatile(dst.offset(offset as isize), val);
            offset += 1;
        }
    }
    dst
}
