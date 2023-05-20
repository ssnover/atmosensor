use crate::drivers;
use crate::static_resources::with_i2c_bus;
use crate::tasks::send_usb_msg;
use crate::tasks::Command;
use crate::tasks::{SensorCommand, UtilityCommand};

static mut LAST_CO2_READING: Option<u16> = None;
static mut LAST_TEMPERATURE_READING: Option<f32> = None;
static mut LAST_HUMIDITY_READING: Option<f32> = None;

pub fn set_measurement_interval(interval_s: u16) {
    let is_successful = unsafe {
        with_i2c_bus(|i2c| {
            let mut scd_sensor = drivers::Scd30::new(i2c);
            scd_sensor.set_measurement_interval(interval_s).is_ok()
        })
    };
    send_usb_msg(&Command::Utility(UtilityCommand::GenericResponse {
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
    send_usb_msg(&Command::Utility(UtilityCommand::GenericResponse {
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
    send_usb_msg(&Command::Utility(UtilityCommand::GenericResponse {
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
    send_usb_msg(&Command::Utility(UtilityCommand::GenericResponse {
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
    send_usb_msg(&Command::Sensor(SensorCommand::ReportNewData));
}

pub fn handle_request_co2_data() {
    let msg = if let Some(last_co2_data) = unsafe { LAST_CO2_READING } {
        Command::Sensor(SensorCommand::LastCO2DataResponse {
            CO2Data: last_co2_data,
        })
    } else {
        Command::Utility(UtilityCommand::GenericResponse { Successful: false })
    };
    send_usb_msg(&msg);
}

pub fn handle_request_temperature() {
    let msg = if let Some(last_temperature_data) = unsafe { LAST_TEMPERATURE_READING } {
        Command::Sensor(SensorCommand::LastTemperatureResponse {
            Temperature: last_temperature_data as i16,
        })
    } else {
        Command::Utility(UtilityCommand::GenericResponse { Successful: false })
    };
    send_usb_msg(&msg);
}

pub fn handle_request_humidity() {
    let msg = if let Some(last_humidity_data) = unsafe { LAST_HUMIDITY_READING } {
        Command::Sensor(SensorCommand::LastHumidityResponse {
            RelativeHumidity: last_humidity_data as u16,
        })
    } else {
        Command::Utility(UtilityCommand::GenericResponse { Successful: false })
    };
    send_usb_msg(&msg);
}
