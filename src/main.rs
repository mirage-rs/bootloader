#![no_std]
#![no_main]
#![feature(global_asm)]

// Bootstrap is only meant to be run on the BPMP.
#[cfg(not(any(target_arch = "arm", rustdoc, test)))]
compile_error!("Please compile the first bootloader stage for ARM7TDMI!");

// Load crt0 from Assembly.
global_asm!(include_str!("crt0.S"));

#[macro_use]
extern crate libtegra;
// Required for memory functions (memset, memcpy, etc) in the assembly code.
extern crate rlibc;

use core::panic::PanicInfo;

use libtegra::{
    gpio,
    pinmux::{PinGrP, PinTristate},
    timer::sleep,
};

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    // TODO: Implement a proper panic handler.
    loop {}
}

fn bring_up_backlight() {
    PinGrP::LcdBlPwmPv0.set_tristate(PinTristate::Passthrough);
    PinGrP::LcdBlEnPv1.set_tristate(PinTristate::Passthrough);

    tegra_gpio!(V, 0).config(gpio::Config::OutputHigh);
    tegra_gpio!(V, 1).config(gpio::Config::OutputHigh);

    sleep(5);

    tegra_gpio!(V, 0).write(gpio::Level::Low);
}

#[no_mangle]
unsafe extern "C" fn main() {
    // Bring up backlight for 5 seconds.
    bring_up_backlight();

    // TODO: Implement application code.
}
