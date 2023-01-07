use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::Pin;
use stm32f1xx_hal::gpio::PushPull;

use crate::static_resources::TEST_LED;

pub fn init(test_led_pin: Pin<'B', 7, Output<PushPull>>) {
    unsafe { TEST_LED.write(test_led_pin) };
}

pub fn enable_test_led() {
    // Get the static singleton and turn it on
    let test_led = unsafe { TEST_LED.assume_init_mut() };
    test_led.set_high();
}

pub fn disable_test_led() {
    let test_led = unsafe { TEST_LED.assume_init_mut() };
    test_led.set_low();
}
