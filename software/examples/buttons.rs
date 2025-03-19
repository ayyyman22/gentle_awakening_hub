use nrf52833_hal::gpio::{Input, PullUp, p0::Parts};
use embedded_hal::digital::v2::InputPin;
use nrf52833_hal::rtc::{Rtc, RtcCompareReg};
use nrf52833_hal::pac;

pub struct Buttons {
    pub alarm_button: Input<PullUp>,
    pub snooze_button: Input<PullUp>,
}

pub fn check_buttons(rtc: &mut Rtc<pac::RTC0>) {
    let mut board = pac::Peripherals::take().unwrap();
    let port0 = Parts::new(board.P0);

    let buttons = Buttons {
        alarm_button: port0.p0_11.into_pullup_input(),
        snooze_button: port0.p0_12.into_pullup_input(),
    };

    if buttons.alarm_button.is_low().unwrap() {
        rtc.set_compare(RtcCompareReg::Compare0, 0).unwrap(); // Stop alarm
    }

    if buttons.snooze_button.is_low().unwrap() {
        rtc.set_compare(RtcCompareReg::Compare0, rtc.get_counter() + 60).unwrap();
    }
}
