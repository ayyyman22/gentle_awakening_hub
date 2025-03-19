use nrf52833_hal::{pwm::Pwm, pac};
use embedded_hal::Pwm as _;

pub fn init_led_pwm(board: &mut pac::Peripherals) -> Pwm<pac::PWM1> {
    Pwm::new(board.PWM1)
}

pub fn set_led_intensity(pwm: &mut Pwm<pac::PWM1>, intensity: u16) {
    let max_duty = pwm.max_duty();
    pwm.set_duty(embedded_hal::Pwm::get_max_duty(&pwm) / 100 * intensity as u16);
}
