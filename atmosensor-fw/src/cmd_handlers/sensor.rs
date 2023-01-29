use crate::drivers;
use crate::static_resources::with_i2c_bus;
use crate::static_resources::USB_RESPONSE_QUEUE;
use crate::tasks::Command;
use crate::tasks::{SensorCommand, UtilityCommand};

static mut LAST_CO2_READING: Option<u16> = None;

pub fn set_measurement_interval(interval_s: u16) {
    let is_successful = unsafe {
        with_i2c_bus(|i2c| {
            let mut scd_sensor = drivers::Scd30::new(i2c);
            scd_sensor.set_measurement_interval(interval_s).is_ok()
        })
    };
    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    let _ = usb_queue.push(Command::Utility(UtilityCommand::GenericResponse {
        Successful: is_successful,
    }));
}

pub fn set_altitude(altitude_m: u16) {
    let is_successful = unsafe {
        with_i2c_bus(|i2c| {
            let mut scd_sensor = drivers::Scd30::new(i2c);
            scd_sensor.set_altitude(altitude_m).is_ok()
        })
    };
    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    let _ = usb_queue.push(Command::Utility(UtilityCommand::GenericResponse {
        Successful: is_successful,
    }));
}

pub fn set_temperature_offset(temp_offset: u16) {
    let is_successful = unsafe {
        with_i2c_bus(|i2c| {
            let mut scd_sensor = drivers::Scd30::new(i2c);
            scd_sensor.set_temperature_offset(temp_offset).is_ok()
        })
    };
    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    let _ = usb_queue.push(Command::Utility(UtilityCommand::GenericResponse {
        Successful: is_successful,
    }));
}

pub fn start_continuous_measurement() {
    let is_successful = unsafe {
        with_i2c_bus(|i2c| {
            let mut scd_sensor = drivers::Scd30::new(i2c);
            scd_sensor.start_measuring().is_ok()
        })
    };
    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    let _ = usb_queue.push(Command::Utility(UtilityCommand::GenericResponse {
        Successful: is_successful,
    }));

    unsafe {
        with_i2c_bus(|i2c| {
            let mut sensor = drivers::Scd30::new(i2c);
            let _ = sensor.read();
        })
    }
}

pub fn handle_data_ready() {
    unsafe {
        with_i2c_bus(|i2c| {
            let mut scd_sensor = drivers::Scd30::new(i2c);
            if let Ok(true) = scd_sensor.data_ready() {
                if let Ok(Some(measurement)) = scd_sensor.read() {
                    LAST_CO2_READING = Some(measurement.co2 as u16);
                }
            }
        })
    };

    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    let _ = usb_queue.push(Command::Sensor(SensorCommand::ReportNewData));
}

pub fn handle_request_co2_data() {
    let usb_queue = unsafe { USB_RESPONSE_QUEUE.assume_init_mut() };
    if let Some(last_co2_data) = unsafe { LAST_CO2_READING } {
        let _ = usb_queue.push(Command::Sensor(SensorCommand::LastCO2DataResponse {
            CO2Data: last_co2_data,
        }));
    } else {
        let _ = usb_queue.push(Command::Utility(UtilityCommand::GenericResponse {
            Successful: false,
        }));
    }
}
