// alarm.rs
use nrf52833_hal as hal;
use hal::pac;
use hal::rtc::Rtc;

static mut RTC_INSTANCE: Option<Rtc<pac::RTC0>> = None;
// You can store the alarm time in a shared static variable.
static mut ALARM_TIME: u32 = 0;

pub fn init(rtc0: pac::RTC0) {
    // Create and start the RTC. (The 32kHz crystal is automatically used if connected.)
    let mut rtc = Rtc::new(rtc0, 0).unwrap();
    rtc.start();
    unsafe {
        RTC_INSTANCE = Some(rtc);
    }
}

pub fn set_rtc(cmd: &str) {
    // Parse a command such as "set_rtc 2025-01-01T07:00:00"
    // Convert the time to RTC ticks and configure the RTC.
    // This is a stub.
}

pub fn get_rtc() {
    // Retrieve the current RTC counter value and send it over CLI.
    // This is a stub.
}

pub fn set_alarm(cmd: &str) {
    // Parse a command such as "set_alarm 07:30:00", convert to RTC ticks,
    // store it in ALARM_TIME, and configure an RTC compare register.
    // This is a stub.
}

pub fn get_alarm() {
    // Report the currently set alarm time.
    // This is a stub.
}

pub fn snooze() {
    // Disable the current alarm and schedule it to trigger after the snooze interval (e.g., 1 minute).
}

pub fn alarm_off() {
    // Stop the alarm: disable blinking, turn off the speaker, update display, etc.
}

pub fn check() {
    // Called from the main loop to check if the current RTC value has reached the alarm time.
    // If so, trigger the gentle wake sequence:
    // 1. Gradually increase LED brightness.
    // 2. Activate speaker output.
    // 3. Update display.
    // For now, this is a stub.
}
