//! The panic / exception handler.

use crate::{mem, EXCEPTION_VECTOR_BASE, PK11_ADDRESS, PK11_SIZE};
use libtegra::{bpmp, fuse};

/// The panic handler of package1.
///
/// The panic handler will
/// - Reset stack pointer and zero out stack
/// - Disable the security engine
/// - Disable fuse programming
/// - Clear PK11 key from memory
/// - Clear PK11 blob from memory
/// - Halt the bpmp
#[no_mangle]
pub extern "C" fn panic_handler() -> ! {
    // Reset the stack pointer
    let stack_top = unsafe { crate::stack_top() as *mut u8 };
    unsafe {
        asm!("ldr sp, {}", in(reg) stack_top as usize);
    }

    // Zero out the stack
    unsafe {
        mem::memset(stack_top..stack_top.offset(0x1000), 0);
    }

    // TODO: Disable security engine

    fuse::disable_programming();

    // TODO: Clear PK11 Key from temporary buffer inmemory

    // Clear PK11 blob from memory
    unsafe {
        let pk11 = PK11_ADDRESS as *mut u8;
        let pk11_range = pk11..pk11.offset(PK11_SIZE as isize);
        mem::memset(pk11_range, 0);
    }

    // Halt the bpmp
    loop {
        bpmp::halt();
    }
}

/// Places the address of the [`panic_handler`] at the
/// exception vector address.
///
/// [`panic_handler`]: ./fn.panic_handler.html
pub fn setup_exception_vector() {
    let ev = EXCEPTION_VECTOR_BASE as *mut u32;
    let panic = panic_handler as *const () as u32;
    unsafe {
        mem::memset(ev..ev.offset(8), panic);
    }
}
