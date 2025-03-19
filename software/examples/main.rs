#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_hal::{
    gpio::p0::Parts,
    pac,
    prelude::*,
    rtc::Rtc,
    pwm::Pwm,
    twim::{Twim, Pins, Frequency},
};
use panic_halt as _;

mod cli;
mod rtc;
mod led;
mod power;
mod sound;
mod buttons;
mod display;
mod alarm;

#[entry]
fn main() -> ! {
    let mut board = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let port0 = Parts::new(board.P0);

    // Clone pins before passing to avoid move errors
    let sda = port0.p0_26.into_floating_input().degrade();
    let scl = port0.p0_27.into_floating_input().degrade();

    // RTC Setup
    let mut rtc = Rtc::new(board.RTC0, 0).unwrap();
    rtc.enable_counter();

    // I2C OLED Setup
    let i2c = Twim::new(
        board.TWIM0,
        Pins { scl, sda },
        Frequency::K400,
    );

    // LED PWM Setup
    let mut led_pwm = Pwm::new(board.PWM1);
    led_pwm.enable();

    // UART Setup
    cli::init_uart(&mut board, &port0);

    // Main Loop
    loop {
        cli::handle_commands(&mut rtc, &mut led_pwm);
        buttons::check_buttons(&mut rtc);
    }
}
