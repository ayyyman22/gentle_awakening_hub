use cortex_m::asm;

pub fn enter_sleep() {
    asm::wfi(); // Wait for interrupt (low power mode)
}
