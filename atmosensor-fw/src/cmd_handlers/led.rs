use crate::static_resources::TEST_LED;

pub fn enable_test_led() {
    // Get the static singleton and turn it on
    let test_led = unsafe { TEST_LED.assume_init_mut() };
    test_led.set_high();
}

pub fn disable_test_led() {
    let test_led = unsafe { TEST_LED.assume_init_mut() };
    test_led.set_low();
}
