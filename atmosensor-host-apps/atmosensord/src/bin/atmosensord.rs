use atmosensor::protocol::{
    Command, DisableTestLed, EnableTestLed, LastCO2DataResponse, LastHumidityResponse,
    LastTemperatureResponse, SetAltitude,
};
use atmosensor_client::{self as atmosensor, Atmosensor};
use chrono::Utc;
use futures::prelude::*;
use influxdb2_derive::WriteDataPoint;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use atmosensord::config::get_config;

#[derive(WriteDataPoint)]
#[measurement = "co2_ppm"]
struct CO2Data {
    #[influxdb(tag)]
    location: String,
    #[influxdb(field)]
    value: u64,
    #[influxdb(timestamp)]
    time: i64,
}

#[derive(WriteDataPoint)]
#[measurement = "temperature_c"]
struct Temperature {
    #[influxdb(tag)]
    location: String,
    #[influxdb(field)]
    value: i64,
    #[influxdb(timestamp)]
    time: i64,
}

#[derive(WriteDataPoint)]
#[measurement = "relative_humidity"]
struct RelativeHumidity {
    #[influxdb(tag)]
    location: String,
    #[influxdb(field)]
    value: u64,
    #[influxdb(timestamp)]
    time: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().expect("Failed to get config");

    env_logger::init();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .unwrap();

    log::info!("Connecting to Atmosensor with config: {:?}", config.device);
    let (mut reader, mut writer) =
        Atmosensor::new(config.device.tty_path.to_string_lossy())?.split();

    log::info!("Connecting to InfluxDB with config: {:?}", config.database);
    let influx_client = config.database.make_client();

    writer
        .send(atmosensor::protocol::Command::SetAltitude(SetAltitude {
            altitude: config.device.altitude,
        }))
        .await?;

    writer
        .send(atmosensor::protocol::Command::StartContinuousMeasurement(
            atmosensor::protocol::StartContinuousMeasurement {},
        ))
        .await?;
    log::info!("Starting continuous measurement");

    let mut led_state = false;

    while running.load(Ordering::SeqCst) {
        match reader
            .receive_next(std::time::Duration::from_millis(500))
            .await
        {
            Some(Command::ReportNewData(_)) => {
                writer
                    .send(Command::RequestLastCO2Data(
                        atmosensor::protocol::RequestLastCO2Data {},
                    ))
                    .await
                    .unwrap();
                writer
                    .send(Command::RequestLastTemperature(
                        atmosensor::protocol::RequestLastTemperature {},
                    ))
                    .await
                    .unwrap();
                writer
                    .send(Command::RequestLastHumidity(
                        atmosensor::protocol::RequestLastHumidity {},
                    ))
                    .await
                    .unwrap();
            }
            Some(Command::LastCO2DataResponse(LastCO2DataResponse { co_2_data })) => {
                let co2_data_points = vec![CO2Data {
                    location: config.device.location.clone(),
                    value: co_2_data.into(),
                    time: Utc::now().timestamp_nanos(),
                }];
                if let Ok(..) = influx_client
                    .write(&config.database.bucket, stream::iter(co2_data_points))
                    .await
                {
                    log::debug!("Writing co2 data... {}", co_2_data);
                }
            }
            Some(Command::LastTemperatureResponse(LastTemperatureResponse { temperature })) => {
                let temp_data_points = vec![Temperature {
                    location: config.device.location.clone(),
                    value: temperature.into(),
                    time: Utc::now().timestamp_nanos(),
                }];
                if let Ok(..) = influx_client
                    .write(&config.database.bucket, stream::iter(temp_data_points))
                    .await
                {
                    log::debug!("Writing temperature data: {}", temperature);
                }
            }
            Some(Command::LastHumidityResponse(LastHumidityResponse { relative_humidity })) => {
                let humidity_data_points = vec![RelativeHumidity {
                    location: config.device.location.clone(),
                    value: relative_humidity.into(),
                    time: Utc::now().timestamp_nanos(),
                }];
                if let Ok(..) = influx_client
                    .write(&config.database.bucket, stream::iter(humidity_data_points))
                    .await
                {
                    log::debug!("Writing relative humidity data: {}", relative_humidity);
                }
            }
            Some(other) => {
                log::warn!("Unhandled command: {other:?}");
            }
            None => {
                log::debug!("Timed out");
                led_state = !led_state;
                if led_state {
                    writer
                        .send(Command::EnableTestLed(EnableTestLed {}))
                        .await
                        .unwrap();
                } else {
                    writer
                        .send(Command::DisableTestLed(DisableTestLed {}))
                        .await
                        .unwrap();
                }
            }
        }
    }

    Ok(())
}
