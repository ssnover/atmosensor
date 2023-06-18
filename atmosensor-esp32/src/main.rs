use anyhow::Result;
use embedded_svc::mqtt::client::QoS;
use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    units::*,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    mqtt::client::{EspMqttClient, MqttClientConfiguration},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::time::Duration;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    mqtt_host: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let app_config = CONFIG;
    let _wifi_handle = atmosensor_esp32::init_wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    let pins = peripherals.pins;
    let i2c_sda = pins.gpio10;
    let i2c_scl = pins.gpio8;
    let bus_frequency = 100_u32;
    let i2c_config = I2cConfig::new().baudrate(bus_frequency.kHz().into());
    let mut i2c = I2cDriver::new(peripherals.i2c0, i2c_sda, i2c_scl, &i2c_config)?;
    let mut scd30 = atmosensor_esp32::scd30::Scd30::new(&mut i2c);
    scd30.set_measurement_interval(5).unwrap();
    scd30.start_measuring().unwrap();
    let _ = scd30.read();

    let mqtt_broker_url = format!("mqtt://{}", app_config.mqtt_host);
    let mqtt_config = MqttClientConfiguration::default();
    let (mut mqtt_client, _conn) = EspMqttClient::new_with_conn(&mqtt_broker_url, &mqtt_config)?;

    loop {
        std::thread::sleep(Duration::from_secs(1));
        if scd30.data_ready().unwrap() {
            if let Ok(Some(measurement)) = scd30.read() {
                let co2_ppm = measurement.co2 as u16;
                let data_buffer = [((co2_ppm & 0xff00) >> 8) as u8, (co2_ppm & 0x00ff) as u8];
                if let Err(err) =
                    mqtt_client.publish("co2_ppm", QoS::ExactlyOnce, true, &data_buffer)
                {
                    log::error!("Unable to publish data: {err:?}");
                }
            } else {
                log::error!("Unable to read data from scd30");
            }
        }
    }
}
