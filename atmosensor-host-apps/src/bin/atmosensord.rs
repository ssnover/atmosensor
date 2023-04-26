use atmosensor::Atmosensor;
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const DEFAULT_TTY: &str = "/dev/ttyACM0";
    let mut args = std::env::args();
    let tty_path = args.nth(1).unwrap_or(DEFAULT_TTY.to_string());

    let port = tokio_serial::new(tty_path, 115200).open_native_async()?;
    let mut atmosensor = Atmosensor::new(port);

    atmosensor.send_bytes(&[0x01, 0x03]).await?;

    Ok(())
}
