#![no_std]
#![no_main]
#![feature(asm, global_asm, lang_items, naked_functions)]

// Bootstrap is only meant to be run on the BPMP.
#[cfg(not(any(target_arch = "arm", rustdoc, test)))]
compile_error!("Please compile the first bootloader stage for ARM7TDMI!");

#[macro_use]
extern crate libtegra;

mod init;
mod memory;
mod panic;

#[cfg(feature = "debug_uart_port")]
use core::fmt::Write;
use core::ops::Range;

use libtegra::se::SecurityEngine;
#[cfg(feature = "debug_uart_port")]
use libtegra::uart::Uart;

use init::init_hardware;

// Load crt0 from Assembly.
global_asm!(include_str!("crt0.S"));

/// The global instance of the Security Engine to be used by the bootloader.
pub const SECURITY_ENGINE: SecurityEngine = SecurityEngine::SE1;

/// The start address of the second-stage bootloader in memory.
///
/// The first-stage bootloader is responsible for loading the second bootloader to
/// this address before passing execution to the TSEC firmware. The TSEC will then
/// decrypt and verify the bootloader at this exact address and pass execution to it.
const BOOTLOADER_START: *mut u32 = 0x4001_6FE0 as *mut _;

/// The size of the second-stage bootloader blob.
///
/// This should be word-aligned to optimize memory copying and clearing operations.
const BOOTLOADER_SIZE: usize = 0x28810;

/// Returns the range of the `.bss` section.
unsafe fn bss_range() -> Range<*mut u32> {
    extern "C" {
        static mut __bss_start__: u32;
        static mut __bss_end__: u32;
    }

    Range {
        start: &mut __bss_start__,
        end: &mut __bss_end__,
    }
}

#[no_mangle]
unsafe extern "C" fn main() {
    // Initialize the hardware from the early bootrom context we're currently in.
    init_hardware().expect("Failed to initialize the hardware!");

    // Zero .bss section.
    memory::memset(bss_range(), 0);

    // Say hello, if debugging is enabled.
    #[cfg(feature = "debug_uart_port")]
    let _ = writeln!(&mut Uart::E, "[Mirage] Hello!");

    // Poison the exception with the panic handler of the bootloader.
    panic::setup_exception_vectors();

    // TODO: Call init methods.
}
