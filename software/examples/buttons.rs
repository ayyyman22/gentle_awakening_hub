// buttons.rs
use nrf52833_hal as hal;
use hal::gpio::{p0::P0_x, Input, PullUp}; // Replace P0_x with actual pin types.
use hal::prelude::*;

// Struct to hold button states.
pub struct Buttons {
    pub alarm_set: bool,
    pub snooze: bool,
}

static mut BUTTONS: Buttons = Buttons {
    alarm_set: false,
    snooze: false,
};

pub fn init<B1, B2>(mut alarm_set_pin: B1, mut snooze_pin: B2)
where
    B1: hal::gpio::Pin<Input<PullUp>>,
    B2: hal::gpio::Pin<Input<PullUp>>,
{
    // Optionally configure interrupts here.
    // For now, we assume the pins are already configured as pull-ups.
    // Save or assign these pins to globals if needed.
}

pub fn process() {
    // Poll the button pins and trigger actions when pressed.
    // Replace the following stubs with actual pin reads:
    let alarm_set_pressed = false; // e.g., alarm_set_pin.is_low().unwrap()
    let snooze_pressed = false;    // e.g., snooze_pin.is_low().unwrap()

    if alarm_set_pressed {
        // Handle Alarm Set button press: perhaps enter a mode to adjust alarm time or turn off alarm.
        crate::alarm::alarm_off();
    }

    if snooze_pressed {
        // Handle Snooze button press: call the snooze functionality.
        crate::alarm::snooze();
    }
}
