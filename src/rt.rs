//! Rust runtime code for hardware bootstrapping in the early boot context.

use core::{mem::size_of, slice};

use crate::memory::clear_mem;

// Include the runtime code written in Assembly that defines the entry symbol for the
// linker, relocates the code accordingly and jumps to the function defined by the
// entrypoint! macro.
global_asm!(include_str!("rt.S"));

/// Defines a Rust entrypoint to the application.
///
/// Due to the early stage in boot, a fair lot of hardware initialization code needs
/// to be executed before generic bootloader tasks really fit. For this reason, the
/// macro defines a Rust entrypoint function that is exported by the link to be called
/// directly from the Assembly runtime component after payload injection.
///
/// The function in question then performs all the hardware initialization and calls a
/// user-provided Rust main function which contains the real bootloader code. This step
/// can be compared to [crt0] in C programs and helps keeping the code clean.
///
/// [crt0]: https://en.wikipedia.org/wiki/Crt0
#[macro_export]
macro_rules! entrypoint {
    ($name:path) => {
        #[export_name = "main"]
        pub unsafe extern "C" fn __entrypoint() {
            // Force the supplied path to have a correct type.
            let func: fn() -> () = $name;

            // Poison the exception vectors using our panic handler.
            $crate::panic::setup_exception_vectors();

            // Clear the .bss segment.
            $crate::rt::clear_bss();

            // Execute the .init_array methods of the binary.
            $crate::rt::call_init_array();

            // TODO: Initialize the hardware.
            //$crate::init::init_hardware().expect("Failed to initialize the hardware!");

            // Jump to the real Rust entrypoint.
            func();

            // Execute the .fini_array methods of the binary.
            $crate::rt::call_fini_array();
        }
    };
}

/// Declares a new function element for the `.init_array` segment.
///
/// The `.init_array` segment is a collection of function pointers
/// that will be called before jumping to the Rust `main` function.
#[macro_export]
macro_rules! init_array {
    ($name:ident $body:expr) => {
        #[allow(dead_code)]
        pub unsafe extern "C" fn $name() {
            #[link_section = ".init_array"]
            #[used]
            static __INIT_ARRAY_ELEMENT: unsafe extern "C" fn() = $name;

            #[inline(always)]
            fn inner() {
                $body
            }

            inner()
        }
    };
}

/// Declares a new function element for the `.fini_array` segment.
///
/// The `.fini_array` segment is a collection of function pointers
/// that will be called after returning from the Rust `main` function.
#[macro_export]
macro_rules! fini_array {
    ($name:ident $body:expr) => {
        #[allow(dead_code)]
        pub unsafe extern "C" fn $name {
            #[link_section = ".fini_array"]
            #[used]
            static __FINI_ARRAY_ELEMENT: unsafe extern "C" fn() = $name;

            #[inline(always)]
            fn inner() {
                $body
            }

            inner()
        }
    };
}

/// Uniformly calls all the functions in the `.init_array` segment.
///
/// The `.init_array` functions of the program must be defined with
/// the [`init_array`] macro to get linked into the segment.
///
/// [`init_array`]: macro.init_array.html
pub unsafe fn call_init_array() {
    extern "C" {
        static __init_array_start__: unsafe extern "C" fn();
        static __init_array_end__: unsafe extern "C" fn();
    }

    // Calculate the amount of pointers that the .init_array segment holds.
    let init_array_length = (&__init_array_end__ as *const _ as usize
        - &__init_array_start__ as *const _ as usize)
        / size_of::<unsafe extern "C" fn()>();

    // Compose a slice of all the function pointers in the segment and call them separately.
    for ptr in slice::from_raw_parts(&__init_array_start__, init_array_length) {
        ptr();
    }
}

/// Uniformly calls all the functions in the `.fini_array` segment.
///
/// The `.fini_array` functions of the program must be defined with
/// the [`fini_array`] macro to get linked into the segment.
///
/// [`fini_array`]: macro.fini_array.html
pub unsafe fn call_fini_array() {
    extern "C" {
        static __fini_array_start__: unsafe extern "C" fn();
        static __fini_array_end__: unsafe extern "C" fn();
    }

    // Calculate the amount of pointers that the .fini_array segment holds.
    let fini_array_length = (&__fini_array_end__ as *const _ as usize
        - &__fini_array_start__ as *const _ as usize)
        / size_of::<unsafe extern "C" fn()>();

    // Compose a slice of all the function pointers in the segment and call them separately.
    for ptr in slice::from_raw_parts(&__fini_array_start__, fini_array_length) {
        ptr();
    }
}

/// Clears the `.bss` segment by overwriting it with zeroes.
pub unsafe fn clear_bss() {
    extern "C" {
        static mut __bss_start__: u32;
        static mut __bss_end__: u32;
    }

    // Overwrite the range of the .bss segment with zeroes.
    clear_mem(&mut __bss_start__ as *mut _..&mut __bss_end__ as *mut _);
}
