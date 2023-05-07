use atmosensor_client::{self as atmosensor, Atmosensor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const DEFAULT_TTY: &str = "/dev/ttyACM0";
    let mut args = std::env::args();
    let tty_path = args.nth(1).unwrap_or(DEFAULT_TTY.to_string());
    let (_reader, mut writer) = Atmosensor::new(tty_path)?.split();

    writer
        .send(atmosensor::protocol::Command::StartContinuousMeasurement(
            atmosensor::protocol::StartContinuousMeasurement {},
        ))
        .await?;

    // Next, add the influx client and a small context for setting up device and reading data

    Ok(())
}
