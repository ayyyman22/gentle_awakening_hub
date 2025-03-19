// main.rs
#![no_std]
#![no_main]

mod cli;
mod alarm;
mod led;
mod power;
mod display;
mod speaker;
mod buttons;

use cortex_m_rt::entry;
use panic_halt as _;
use nrf52833_hal as hal;
use hal::pac;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    // Take ownership of the device peripherals.
    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    // Initialize LED on a chosen pin (example: P0.17)
    let led_pin = port0.p0_17.into_push_pull_output(hal::gpio::Level::Low);
    led::init(led_pin);

    // Initialize RTC and Alarm (using RTC0; the 32kHz crystal is assumed to be connected as per board design).
    alarm::init(p.RTC0);

    // Initialize CLI (e.g., via UART on pins P0.06 and P0.08).
    cli::init();

    // Initialize OLED display via I2C (example pins P0.14=SCL, P0.15=SDA).
    display::init(p.TWIM0, port0.p0_14, port0.p0_15);

    // Initialize Speaker (for wake-up sound) on an appropriate PWM channel.
    speaker::init();

    // Initialize buttons (Alarm Set and Snooze).
    buttons::init(
        port0.p0_20.into_pullup_input(), // Example: Alarm Set Button
        port0.p0_21.into_pullup_input(), // Example: Snooze Button
    );

    loop {
        // Process any incoming CLI commands.
        cli::process();

        // Check if it’s time to trigger the alarm or pre-wake sequence.
        alarm::check();

        // Update the OLED display with current time and alarm settings.
        display::update();

        // Poll button states and handle their events.
        buttons::process();

        // Manage power state (enter low-power mode if idle).
        power::manage();
    }
}