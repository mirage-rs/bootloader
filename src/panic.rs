//! The panic / exception handler.

use crate::{mem, PK11_ADDRESS, PK11_SIZE};
use libtegra::{bpmp, fuse};

#[no_mangle]
unsafe extern "C" fn panic_handler() {
    // Reset the stack pointer
    let stack_top = crate::stack_top() as *mut u8;
    asm!("ldr sp, {}", in(reg) stack_top as usize);

    // Zero out the stack
    mem::memset(stack_top..stack_top.offset(0x1000), 0);

    // TODO: Disable security engine

    fuse::disable_programming();

    // TODO: Clear PK11 Key from temporary buffer inmemory

    // Clear PK11 blob from memory
    let pk11 = PK11_ADDRESS as *mut u8;
    let pk11_range = pk11..pk11.offset(PK11_SIZE as isize);
    mem::memset(pk11_range, 0);

    // Halt the bpmp
    loop {
        bpmp::halt();
    }
}
