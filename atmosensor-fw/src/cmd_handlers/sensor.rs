use crate::static_resources::with_i2c_bus;
use crate::static_resources::USB_RESPONSE_QUEUE;
use crate::tasks::Command;
use crate::tasks::UtilityCommand;

pub fn set_measurement_interval(interval_s: u16) {
    let is_successful = unsafe { with_i2c_bus(|i2c| {
        let mut scd_sensor = scd30::scd30::Scd30::new(i2c);
        scd_sensor.set_measurement_interval(interval_s).is_ok()
    })};
    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    let _ = usb_queue.push(Command::Utility(UtilityCommand::GenericResponse {
        Successful: is_successful,
    }));
}
