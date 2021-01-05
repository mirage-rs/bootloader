#![no_std]
#![no_main]
#![feature(asm, global_asm, lang_items, naked_functions)]

// Bootloader code is only meant to be run on the BPMP.
#[cfg(not(any(target_arch = "arm", rustdoc, test)))]
compile_error!("Please compile the first bootloader stage for ARM7TDMI!");

#[macro_use]
extern crate libtegra;

mod init;
mod memory;
mod panic;
#[allow(dead_code)]
#[macro_use]
mod rt;

#[cfg(feature = "debug_uart_port")]
use core::fmt::Write;

use libtegra::gpio;
use libtegra::pinmux::{PinGrP, PinTristate};
use libtegra::se::SecurityEngine;
use libtegra::timer::sleep;
#[cfg(feature = "debug_uart_port")]
use libtegra::uart::Uart;

entrypoint!(main);

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

fn bring_up_backlight() {
    unsafe {
        PinGrP::LcdBlPwmPv0.set_tristate(PinTristate::Passthrough);
        PinGrP::LcdBlEnPv1.set_tristate(PinTristate::Passthrough);
    }

    tegra_gpio!(V, 0).config(gpio::Config::OutputHigh);
    tegra_gpio!(V, 1).config(gpio::Config::OutputHigh);

    sleep(5);

    tegra_gpio!(V, 0).write(gpio::Level::Low);
}

fn main() {
    // Say hello, if debugging is enabled.
    #[cfg(feature = "debug_uart_port")]
    let _ = writeln!(&mut Uart::E, "[Mirage] Hello!");

    // Bring up backlight for debugging.
    bring_up_backlight();
}
