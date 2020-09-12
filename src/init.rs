//! Hardware initialization code for the Tegra X1 for the early bootrom context imposed
//! by the RCM exploit CVE-2018-6242.

use libtegra::{car, fuse, mc, pmc, timer};

fn config_oscillators(car: &car::Registers, pmc: &pmc::Registers) {
    let sysctr0 = unsafe { &*pmc::counter0::REGISTERS };
    let timer = unsafe { &*timer::timerus::REGISTERS };

    // Set CLK_M_DIVISOR t o2.
    car
        .CLK_RST_CONTROLLER_SPARE_REG0_0
        .set((car.CLK_RST_CONTROLLER_SPARE_REG0_0.get() & 0xFFFF_FFF3) | 0x4);
    // Set counter frequency.
    sysctr0.SYSCTR0_CNTFID0_0.set(0x124F800);
    // Set 19.2MHz clk_m.
    timer.TIMERUS_USEC_CFG_0.set(0x45F);
    // Set OSC to 38.4MHz and drive strength.
    car.CLK_RST_CONTROLLER_OSC_CTRL_0.set(0x5000_0071);

    // Set LP0 OSC drive strength.
    pmc
        .APBDEV_PMC_OSC_EDPD_OVER_0
        .set((pmc.APBDEV_PMC_OSC_EDPD_OVER_0.get() & 0xFFFF_FF81) | 0xE);
    pmc
        .APBDEV_PMC_OSC_EDPD_OVER_0
        .set((pmc.APBDEV_PMC_OSC_EDPD_OVER_0.get() & 0xFFBF_FFFF) | 0x400000);
    pmc
        .APBDEV_PMC_CNTRL2_0
        .set((pmc.APBDEV_PMC_CNTRL2_0.get() & 0xFFFF_EFFF) | 0x1000);
    // LP0 EMC2TMC_CFG_XM2COMP_PU_VREF_SEL_RANGE.
    pmc
        .APBDEV_PMC_SCRATCH188_0
        .set((pmc.APBDEV_PMC_SCRATCH188_0.get() & 0xFCFF_FFFF) | 0x2000000);

    // Set HCLK div to 2 and PCLK div to 1.
    car.CLK_RST_CONTROLLER_CLK_SYSTEM_RATE_0.set(0x10);
    // PLLMB disable.
    car
        .CLK_RST_CONTROLLER_PLLMB_BASE_0
        .set(car.CLK_RST_CONTROLLER_PLLMB_BASE_0.get() & 0xBFFF_FFFF);

    // 0x249F = 19200000 * (16 / 32.768 KHz)
    pmc
        .APBDEV_PMC_TSC_MULT_0
        .set((pmc.APBDEV_PMC_TSC_MULT_0.get() & 0xFFFF_0000) | 0x249F);

    // Set BPMP/SCLK div to 1.
    car.CLK_RST_CONTROLLER_CLK_SYSTEM_RATE_0.set(0);
    // Set BPMP/SCLK source to Run and PLLP_OUT2 (204MHz).
    car.CLK_RST_CONTROLLER_SCLK_BURST_POLICY_0.set(0x2000_4444);
    // Enable SUPER_SDIV to 1.
    car.CLK_RST_CONTROLLER_SUPER_SCLK_DIVIDER_0.set(0x8000_0000);
    // Set HCLK div to 1 and PCLK div to 3.
    car.CLK_RST_CONTROLLER_CLK_SYSTEM_RATE_0.set(0x2);
}

/// Performs hardware initialization for the Tegra X1 SoC.
pub fn init_hardware() {
    let car = unsafe { &*car::REGISTERS };
    let pmc = unsafe { &*pmc::REGISTERS };

    // Reboot the Security Engine.
    car::Clock::SE.enable();

    // Initialize the fuse driver by making the registers visible, disabling
    // the private key and disabling programming.
    fuse::init();

    // Enable clocks to Memory Controllers and disable AHB redirect.
    mc::enable_mc();

    // Initialize counters, CLKM, BPMP and other clocks based on 38.4MHz oscillator.
    config_oscillators(car, pmc);
}
