use nrf52833_hal::{pwm::{Pwm}, rtc::{Rtc}, pac};

pub fn handle_alarm(rtc: &mut Rtc<pac::RTC0>, pwm: &mut Pwm<pac::PWM0>) {
    let alarm_time = rtc.get_counter();
    if rtc.get_counter() >= alarm_time {
        pwm.set_duty(pwm.max_duty() / 2);
        pwm.enable();
    }
}

pub fn blink_led(pwm: &mut Pwm<pac::PWM1>) {
    let max_duty = pwm.max_duty();
    pwm.set_duty(max_duty / 10);
}
