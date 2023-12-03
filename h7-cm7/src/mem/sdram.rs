// All of this is mostly stolen from https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/fmc.rs

// The SDRAM chip on the default configuration of the Portenta H7 is 8MiB
pub const SDRAM_SIZE: usize = 8 * 1024 * 1024;

// Refer to ARM®v7-M Architecture Reference Manual ARM DDI 0403
// Version E.b Section B3.5
const MEMFAULTENA: u32 = 1 << 16;

const REGION_NUMBER0: u32 = 0x00;
const REGION_BASE_ADDRESS: u32 = 0xD000_0000;
const REGION_FULL_ACCESS: u32 = 0x03;
const REGION_CACHEABLE: u32 = 0x01;
const REGION_WRITE_BACK: u32 = 0x01;
const REGION_ENABLE: u32 = 0x01;

const MPU_ENABLE: u32 = 0x01;
const MPU_DEFAULT_MMAP_FOR_PRIVILEGED: u32 = 0x04;

/// Configre a pin for the FMC controller
#[macro_export]
macro_rules! fmc_pins {
    ($($pin:expr),*) => {
        (
            $(
                $pin.into_push_pull_output()
                    .speed(stm32h7xx_hal::gpio::Speed::VeryHigh)
                    .into_alternate::<12>()
                    .internal_pull_up(true)
            ),*
        )
    };
}

pub fn configure(mpu: &cortex_m::peripheral::MPU, scb: &cortex_m::peripheral::SCB) {
    cortex_m::asm::dmb();
    unsafe {
        scb.shcsr.modify(|r| r & !MEMFAULTENA);
        mpu.ctrl.write(0);
        mpu.rnr.write(REGION_NUMBER0);
        mpu.rbar.write(REGION_BASE_ADDRESS);
        mpu.rasr
            .write((REGION_FULL_ACCESS << 24) | (REGION_CACHEABLE << 17) | (REGION_WRITE_BACK << 16) | ((SDRAM_SIZE.ilog2() - 1) << 1) | REGION_ENABLE);
        mpu.ctrl.modify(|r| r | MPU_DEFAULT_MMAP_FOR_PRIVILEGED | MPU_ENABLE);

        scb.shcsr.modify(|r| r | MEMFAULTENA);
    }
    // Ensure MPU settings take effect
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}
