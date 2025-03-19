use nrf52833_hal::{rtc::{Rtc, RtcCompareReg}, pac};

pub fn init_rtc(board: &mut pac::Peripherals) -> Rtc<pac::RTC0> {
    let mut rtc = Rtc::new(board.RTC0, 0).unwrap();
    rtc.enable_counter();
    rtc
}

pub fn set_alarm(rtc: &mut Rtc<pac::RTC0>, seconds: u32) {
    rtc.set_compare(RtcCompareReg::Compare0, rtc.get_counter() + seconds).unwrap();
}
