// power.rs
use nrf52833_hal as hal;
use cortex_m::asm;

pub fn manage() {
    // Insert conditions to enter low-power mode if no activity is required.
    asm::wfi();
}

pub fn enter_deep_sleep() -> ! {
    // Enter deep sleep (System OFF). HAL APIs can be used for full System OFF.
    loop {
        asm::wfi();
    }
}
