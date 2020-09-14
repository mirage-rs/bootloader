//! Implementations of functions related to panic and exception handling in
//! the early boot stage.

#[cfg(feature = "debug_uart_port")]
use core::fmt::Write;
use core::panic::PanicInfo;

use libtegra::memory_map::EXCEPTION_VECTORS;
#[cfg(feature = "debug_uart_port")]
use libtegra::uart::Uart;
use libtegra::{bpmp, fuse};

use crate::memory;
use crate::SECURITY_ENGINE;
use crate::{BOOTLOADER_SIZE, BOOTLOADER_START};

extern "C" {
    static mut __stack_start__: u32;
    static mut __stack_end__: u32;
}

/// Implementation of the panic handler for the bootloader.
///
/// The panic handler is either called when a Rust-side panic is hit through
/// a more idiomatic wrapper or through the ARM exception vectors which will
/// be poisoned with a pointer to this function.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn panic_handler() -> ! {
    // Reset the stack pointer.
    let stack_bottom: *mut u32 = &mut __stack_end__;
    asm!("mov sp, {}", in(reg) stack_bottom as usize);

    // Clear the stack without overwriting the return address of `clear_mem`.
    // XXX: Nintendo hardcodes a stack limit of 0x1000. Should we use the real stack offset?
    let stack_top: *mut u32 = &mut __stack_start__;
    memory::clear_mem(stack_top..stack_bottom.offset(-1));

    // Disable the Security Engine.
    SECURITY_ENGINE.disable();

    // Disable fuse programming until next reboot.
    fuse::disable_programming();

    // Clear the second-stage bootloader from memory.
    memory::clear_mem(BOOTLOADER_START..BOOTLOADER_START.offset(BOOTLOADER_SIZE as isize));

    // Halt the Boot and Power Management processor.
    loop {
        bpmp::halt();
    }
}

/// The exception handling personality function used by the bootloader.
///
/// Since there is no exception handling done here, this function does nothing
/// and should also never get called.
#[cfg(target_os = "none")]
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
    // Purposefully do nothing.
}

/// Implementation of the panic function for the bootloader.
///
/// In case something went really wrong, log a message over UART and execute the
/// lower-level [`panic_handler`] that is also invoked through ARM exception vectors.
///
/// [`panic_handler`]: fn.panic_handler.html
#[cfg(target_os = "none")]
#[no_mangle]
#[panic_handler]
pub extern fn panic(_info: &PanicInfo<'_>) -> ! {
    #[cfg(feature = "debug_uart_port")]
    let _ = writeln!(&mut Uart::E, "[Mirage] Rust panicked: {}", _info);

    unsafe { panic_handler() }
}

/// Poisons the exception vectors of the BPMP with the lower-level [`panic_handler`]
/// implementation of the bootloader.
///
/// [`panic_handler`]: fn.panic_handler.html
pub fn setup_exception_vectors() {
    let ev = EXCEPTION_VECTORS as *mut u32;
    let panic = panic_handler as *const () as u32;

    unsafe {
        memory::memset(ev..ev.offset(8), panic);
    }
}
