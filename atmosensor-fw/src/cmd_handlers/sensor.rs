use crate::static_resources::I2C_BUS;
use crate::static_resources::USB_RESPONSE_QUEUE;
use crate::tasks::Command;
use crate::tasks::UtilityCommand;

pub fn set_measurement_interval(interval_s: u16) {
    let i2c = unsafe { I2C_BUS.assume_init_mut() };
    let mut scd_sensor = scd30::scd30::Scd30::new(i2c);
    let is_successful = scd_sensor.set_measurement_interval(interval_s).is_ok();
    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    let _ = usb_queue.push(Command::Utility(UtilityCommand::GenericResponse {
        Successful: is_successful,
    }));
}
