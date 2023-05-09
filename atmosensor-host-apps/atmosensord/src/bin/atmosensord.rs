use atmosensor::protocol::{Command, LastCO2DataResponse};
use atmosensor_client::{self as atmosensor, Atmosensor};
use chrono::Utc;
use futures::prelude::*;
use influxdb2_derive::WriteDataPoint;

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

struct InfluxConfig<'a> {
    org: &'a str,
    bucket: &'a str,
    url: &'a str,
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    const DEFAULT_TTY: &str = "/dev/ttyACM0";
    let mut args = std::env::args();
    let tty_path = args.nth(1).unwrap_or(DEFAULT_TTY.to_string());
    let (mut reader, mut writer) = Atmosensor::new(tty_path)?.split();

    let influx_cfg = initialize_influx_config();
    let influx_client = influxdb2::Client::new(influx_cfg.url, influx_cfg.org, &influx_cfg.token);

    writer
        .send(atmosensor::protocol::Command::StartContinuousMeasurement(
            atmosensor::protocol::StartContinuousMeasurement {},
        ))
        .await?;

    loop {
        match reader.receive().await {
            Command::ReportNewData(_) => {
                writer
                    .send(Command::RequestLastCO2Data(
                        atmosensor::protocol::RequestLastCO2Data {},
                    ))
                    .await
                    .unwrap();
            }
            Command::LastCO2DataResponse(LastCO2DataResponse { co_2_data }) => {
                let co2_data_points = vec![CO2Data {
                    location: "living_room".into(),
                    value: co_2_data.into(),
                    time: Utc::now().timestamp_nanos(),
                }];
                if let Ok(..) = influx_client
                    .write(influx_cfg.bucket, stream::iter(co2_data_points))
                    .await
                {
                    log::info!("Writing data... {}", co_2_data);
                }
            }
            other => {
                log::warn!("Unhandled command: {other:?}");
            }
        }
    }
}

fn initialize_influx_config() -> InfluxConfig<'static> {
    InfluxConfig {
        org: "snostorm",
        bucket: "homelab",
        url: "http://localhost:8086",
        token: std::env::var("INFLUXDB2_TOKEN").unwrap(),
    }
}
