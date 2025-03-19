use nrf52833_hal::{pwm::Pwm, pac};
use embedded_hal::Pwm as _;

pub fn init_speaker(board: &mut pac::Peripherals) -> Pwm<pac::PWM0> {
    Pwm::new(board.PWM0)
}

pub fn play_alarm(pwm: &mut Pwm<pac::PWM0>) {
    let max_duty = pwm.max_duty();
    pwm.set_duty(embedded_hal::Pwm::get_max_duty(&pwm) / 2);
    pwm.enable();
}

pub fn stop_alarm(pwm: &mut Pwm<pac::PWM0>) {
    pwm.disable();
}
