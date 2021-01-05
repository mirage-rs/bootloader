//! Hardware initialization for the NVIDIA Tegra X1.

use libtegra::i2c::{Error, I2c};
use libtegra::pinmux::{
    PinFunction, PinGrP, PinIo, PinIoHv as PinEIoHv, PinLock, PinOd, PinPull, PinTristate,
};
#[cfg(feature = "debug_uart_port")]
use libtegra::uart::{Uart, BAUD_115200};
use libtegra::{apb, car, fuse, gpio, mc, pmc, timer};

const MAX77620_PWR: u32 = 0x3C;

// TODO: Configure remaining GPIOs for the advanced stages of the system here?
const GPIO_CONFIG: [(gpio::Gpio, gpio::Config); 6] = [
    (tegra_gpio!(D, 1), gpio::Config::Input), // Pin mode for Joy-Con IsAttached and UART-C TX
    (tegra_gpio!(E, 6), gpio::Config::Input), // Joy-Con IsAttached mode
    (tegra_gpio!(G, 0), gpio::Config::Input), // Pin mode for Joy-Con IsAttached and UART-B TX
    (tegra_gpio!(H, 6), gpio::Config::Input), // Joy-Con IsAttached mode
    (tegra_gpio!(X, 6), gpio::Config::Input), // Volume Up
    (tegra_gpio!(X, 7), gpio::Config::Input), // Volume Down
];

const PIN_CONFIG: [(
    PinGrP,
    PinFunction,
    PinPull,
    PinTristate,
    PinIo,
    PinLock,
    PinOd,
    PinEIoHv,
); 12] = [
    // UART-A TX
    (
        PinGrP::Uart1TxPu0,
        PinFunction::Uarta,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // UART-A RX
    (
        PinGrP::Uart1RxPu1,
        PinFunction::Uarta,
        PinPull::Up,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // UART-A RTS
    (
        PinGrP::Uart1RtsPu2,
        PinFunction::Uarta,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // UART-A CTS
    (
        PinGrP::Uart1CtsPu3,
        PinFunction::Uarta,
        PinPull::Down,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // UART-B TX
    (
        PinGrP::Uart2TxPg0,
        PinFunction::Uartb,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // UART-C TX
    (
        PinGrP::Uart3TxPd1,
        PinFunction::Uartc,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // GPIO PE6
    (
        PinGrP::Pe6,
        PinFunction::Default,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // GPIO PH6
    (
        PinGrP::Ph6,
        PinFunction::Default,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    // I2C-1 SCL
    (
        PinGrP::Gen1I2CSclPj1,
        PinFunction::I2C1,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    // I2C-1 SDA
    (
        PinGrP::Gen1I2CSdaPj0,
        PinFunction::I2C1,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    // I2C-5 SCL
    (
        PinGrP::PwrI2CSclPy3,
        PinFunction::I2Cpmu,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    // I2C-5 SDA
    (
        PinGrP::PwrI2CSdaPy4,
        PinFunction::I2Cpmu,
        PinPull::None,
        PinTristate::Passthrough,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
];

fn config_oscillators(car: &car::Registers, pmc: &pmc::Registers) {
    let sysctr0 = unsafe { &*pmc::counter0::REGISTERS };
    let timer = unsafe { &*timer::timerus::REGISTERS };

    // Set CLK_M_DIVISOR to 2.
    car.CLK_RST_CONTROLLER_SPARE_REG0_0
        .set((car.CLK_RST_CONTROLLER_SPARE_REG0_0.get() & 0xFFFF_FFF3) | 0x4);
    // Set counter frequency.
    sysctr0.SYSCTR0_CNTFID0_0.set(19200000);
    // Set 19.2MHz clk_m.
    timer.TIMERUS_USEC_CFG_0.set(0x45F);
    // Set OSC to 38.4MHz and drive strength.
    car.CLK_RST_CONTROLLER_OSC_CTRL_0.set(0x5000_0071);

    // Set LP0 OSC drive strength.
    pmc.APBDEV_PMC_OSC_EDPD_OVER_0
        .set((pmc.APBDEV_PMC_OSC_EDPD_OVER_0.get() & 0xFFFF_FF81) | 0xE);
    pmc.APBDEV_PMC_OSC_EDPD_OVER_0
        .set((pmc.APBDEV_PMC_OSC_EDPD_OVER_0.get() & 0xFFBF_FFFF) | 0x400000);
    pmc.APBDEV_PMC_CNTRL2_0
        .set((pmc.APBDEV_PMC_CNTRL2_0.get() & 0xFFFF_EFFF) | 0x1000);
    // LP0 EMC2TMC_CFG_XM2COMP_PU_VREF_SEL_RANGE.
    pmc.APBDEV_PMC_SCRATCH188_0
        .set((pmc.APBDEV_PMC_SCRATCH188_0.get() & 0xFCFF_FFFF) | 0x2000000);

    // Set HCLK div to 2 and PCLK div to 1.
    car.CLK_RST_CONTROLLER_CLK_SYSTEM_RATE_0.set(0x10);
    // PLLMB disable.
    car.CLK_RST_CONTROLLER_PLLMB_BASE_0
        .set(car.CLK_RST_CONTROLLER_PLLMB_BASE_0.get() & 0xBFFF_FFFF);

    // 0x249F = 19200000 * (16 / 32.768 KHz)
    pmc.APBDEV_PMC_TSC_MULT_0
        .set((pmc.APBDEV_PMC_TSC_MULT_0.get() & 0xFFFF_0000) | 0x249F);

    // Set BPMP/SCLK div to 1.
    car.CLK_RST_CONTROLLER_CLK_SOURCE_SYS_0.set(0);
    // Set BPMP/SCLK source to Run and PLLP_OUT2 (204MHz).
    car.CLK_RST_CONTROLLER_SCLK_BURST_POLICY_0.set(0x2000_4444);
    // Enable SUPER_SDIV to 1.
    car.CLK_RST_CONTROLLER_SUPER_SCLK_DIVIDER_0.set(0x8000_0000);
    // Set HCLK div to 1 and PCLK div to 3.
    car.CLK_RST_CONTROLLER_CLK_SYSTEM_RATE_0.set(0x2);
}

fn config_pinmux(apb_misc: &apb::misc::AmbaPeripheralBus) {
    // Clamp inputs when tristated.
    apb_misc.pp.APB_MISC_PP_PINMUX_GLOBAL_0_0.set(0);

    // Configure the GPIOs.
    for entry in GPIO_CONFIG.iter() {
        entry.0.config(entry.1);
    }

    // Configure the pin multiplexing.
    for entry in PIN_CONFIG.iter() {
        unsafe {
            entry.0.set_function(entry.1);
            entry.0.set_pull(entry.2);
            entry.0.set_tristate(entry.3);
            entry.0.set_io(entry.4);
            entry.0.set_lock(entry.5);
            entry.0.set_od(entry.6);
            entry.0.set_io_hv(entry.7);
        }
    }
}

fn config_pmc_scratch(pmc: &pmc::Registers) {
    // Unset Debug console from Customer Option.
    pmc.APBDEV_PMC_SCRATCH20_0
        .set(pmc.APBDEV_PMC_SCRATCH20_0.get() & 0xFFF3_FFFF);
    // Unset DATA_DQ_E_IVREF EMC_PMACRO_DATA_PAD_TX_CTRL.
    pmc.APBDEV_PMC_SCRATCH190_0
        .set(pmc.APBDEV_PMC_SCRATCH190_0.get() & 0xFFFF_FFFE);
    // Disable the fuse private key.
    pmc.APBDEV_PMC_SECURE_SCRATCH21_0
        .set(pmc.APBDEV_PMC_SECURE_SCRATCH21_0.get() | 0x10);
}

/// Performs hardware initialization for the Tegra X1 SoC.
pub fn init_hardware() -> Result<(), Error> {
    let apb_misc = unsafe { &*apb::misc::REGISTERS };
    let car = unsafe { &*car::REGISTERS };
    let pmc = unsafe { &*pmc::REGISTERS };

    // Clear the boot reason to avoid problems later on.
    pmc.APBDEV_PMC_SCRATCH200_0.set(0);
    pmc.APBDEV_PMC_RST_STATUS_0.set(0);
    apb_misc
        .pp
        .APB_MISC_PP_STRAPPING_OPT_A_0
        .set(apb_misc.pp.APB_MISC_PP_STRAPPING_OPT_A_0.get() & 0xF0);
    apb_misc
        .pp
        .APB_MISC_PP_STRAPPING_OPT_A_0
        .modify(apb::misc::misc_pp::APB_MISC_PP_STRAPPING_OPT_A_0::RCM_STRAPS.val(7));

    // Reboot the Security Engine.
    car::Clock::SE.enable();

    // Initialize the fuse driver by making the registers visible, disabling
    // the private key and disabling programming.
    fuse::init();

    // Enable clocks to Memory Controllers and disable AHB redirect.
    mc::enable_mc();

    // Initialize counters, CLKM, BPMP and other clocks based on 38.4MHz oscillator.
    config_oscillators(car, pmc);

    // Initialize the SoC pin configurations.
    config_pinmux(apb_misc);

    // Initialize UART E for debugging, if desired.
    #[cfg(feature = "debug_uart_port")]
    Uart::E.init(BAUD_115200);

    // Reboot the Dynamic Voltage and Frequency Scaling device.
    car::Clock::CL_DVFS.enable();

    // Reboot the TZRAM device.
    car::Clock::TZRAM.enable();

    // Initialize the I2C1 and I2CPWR devices.
    I2c::C1.init();
    I2c::C5.init();

    // Configure the PMIC.
    I2c::C5.write_byte(MAX77620_PWR, 0x04, 0x40)?;

    I2c::C5.write_byte(MAX77620_PWR, 0x41, 0x60)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x43, 0x38)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x44, 0x3A)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x45, 0x38)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x4A, 0x0F)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x4E, 0xC7)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x4F, 0x4F)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x50, 0x29)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x52, 0x1B)?;
    I2c::C5.write_byte(MAX77620_PWR, 0x56, 0x22)?;

    // Configure SD0 voltage.
    I2c::C5.write_byte(MAX77620_PWR, 0x16, 0x2A)?;

    // Configure and lock PMC scratch registers.
    // XXX: Starting from 4.0.0+, this was removed.
    config_pmc_scratch(pmc);

    // Set SCLK to PLLP_OUT (408MHz).
    car.CLK_RST_CONTROLLER_SCLK_BURST_POLICY_0.set(0x2000_3333);

    Ok(())
}
