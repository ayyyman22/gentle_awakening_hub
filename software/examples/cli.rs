use nrf52833_hal::{
    pac,
    uarte::{Baudrate, Parity, Pins, Uarte},
    pwm::Pwm,
    rtc::Rtc,
    gpio::p0::Parts,
    gpio::Level,
};

pub fn init_uart(board: &mut pac::Peripherals, port0: &Parts) {
    let pins = Pins {
        txd: port0.p0_06.into_push_pull_output(Level::Low).degrade(),
        rxd: port0.p0_08.into_floating_input().degrade(),
        cts: None,
        rts: None,
    };

    let _uart = Uarte::new(board.UARTE0, pins, Parity::EXCLUDED, Baudrate::BAUD115200);
}

pub fn handle_commands(rtc: &mut Rtc<pac::RTC0>, led_pwm: &mut Pwm<pac::PWM1>) {
    led_pwm.set_duty(0, embedded_hal::Pwm::get_max_duty(&led_pwm) / 2);
}
