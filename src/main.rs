#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

// Bootstrap is only meant to be run on the BPMP.
#[cfg(not(any(target_arch = "arm", rustdoc, test)))]
compile_error!("Please compile the first bootloader stage for ARM7TDMI!");

#[macro_use]
extern crate libtegra;

mod init;
mod mem;
mod panic;

use core::{ops::Range, panic::PanicInfo};

use init::init_hardware;

// Load crt0 from Assembly.
global_asm!(include_str!("crt0.S"));

/// The buffer where the PK11 key is located
const KEY_BUFFER: *const u8 = 0x40013720 as *const _;

/// The address of the PK11 blob
const PK11_ADDRESS: *const u8 = 0x40016FE0 as *const _;
/// The size of the PK11 blob.
const PK11_SIZE: usize = 0x28810;
/// The base address of the exception vectors.
const EXCEPTION_VECTOR_BASE: usize = 0x6000F200;

/// Returns the range of the .bss section.
pub unsafe fn bss_range() -> Range<*mut u32> {
    extern "C" {
        static mut __bss_start__: u32;
        static mut __bss_end__: u32;
    }

    Range {
        start: &mut __bss_start__,
        end: &mut __bss_end__,
    }
}

/// Returns a pointer to the top of the stack.
pub unsafe fn stack_top() -> *const u8 {
    extern "C" {
        static __stack_top__: u32;
    }

    __stack_top__ as *const u8
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    panic::panic_handler()
}

#[no_mangle]
unsafe extern "C" fn main() {
    // Initialize the hardware from the early bootrom context we're currently in.
    init_hardware();

    // Zero .bss section.
    mem::memset(bss_range(), 0);

    // Setup the panic_handler.
    panic::setup_exception_vector();

    // TODO: Call init methods.
}
