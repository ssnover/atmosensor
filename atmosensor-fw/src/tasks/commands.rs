fn read_u16(buf: &[u8]) -> u16 {
    ((buf[0] as u16) << 8) | buf[1] as u16
}

#[derive(Copy, Clone)]
pub enum Command {
    Nop,
    Sensor(SensorCommand),
    Utility(UtilityCommand),
}

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub enum SensorCommand {
    SetMeasurementInterval { MeasurementInterval: u16 },
    SetAltitude { Altitude: u16 },
    SetTemperatureOffset { TemperatureOffset: u16 },
    StartContinuousMeasurement,
    ReportNewData,
    RequestLastCO2Data,
    LastCO2DataResponse { CO2Data: u16 },
    RequestLastTemperature,
    LastTemperatureResponse { Temperature: i16 },
    RequestLastHumidity,
    LastHumidityResponse { RelativeHumidity: u16 },
}

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub enum UtilityCommand {
    EnableTestLed,
    DisableTestLed,
    GenericResponse { Successful: bool },
}

impl Command {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            0xaa => Some(Command::Utility(UtilityCommand::from_bytes(&buf[1..])?)),
            0x01 => Some(Command::Sensor(SensorCommand::from_bytes(&buf[1..])?)),
            _ => None,
        }
    }

    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<usize, ()> {
        if let Ok(bytes) = match self {
            Command::Utility(cmd) => {
                buf[0] = 0xaa;
                cmd.to_bytes(&mut buf[1..])
            }
            Command::Sensor(cmd) => {
                buf[0] = 0x01;
                cmd.to_bytes(&mut buf[1..])
            }
            _ => Err(()),
        } {
            Ok(bytes + 1)
        } else {
            Err(())
        }
    }
}

impl UtilityCommand {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            0x00 => Some(UtilityCommand::EnableTestLed),
            0x01 => Some(UtilityCommand::DisableTestLed),
            _ => None,
        }
    }

    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<usize, ()> {
        match self {
            UtilityCommand::GenericResponse { Successful } => {
                buf[0] = 0x02;
                buf[1] = if *Successful { 0x01 } else { 0x00 };
                Ok(2)
            }
            _ => return Err(()),
        }
    }
}

impl SensorCommand {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            0x00 => Some(SensorCommand::SetMeasurementInterval {
                MeasurementInterval: read_u16(&buf[1..=2]),
            }),
            0x01 => Some(SensorCommand::SetAltitude {
                Altitude: read_u16(&buf[1..=2]),
            }),
            0x02 => Some(SensorCommand::SetTemperatureOffset {
                TemperatureOffset: read_u16(&buf[1..=2]),
            }),
            0x03 => Some(SensorCommand::StartContinuousMeasurement),
            0x05 => Some(SensorCommand::RequestLastCO2Data),
            0x07 => Some(SensorCommand::RequestLastTemperature),
            0x09 => Some(SensorCommand::RequestLastHumidity),
            _ => None,
        }
    }

    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<usize, ()> {
        match self {
            SensorCommand::ReportNewData => {
                buf[0] = 0x04;
                Ok(1)
            }
            SensorCommand::LastCO2DataResponse { CO2Data } => {
                buf[0] = 0x06;
                buf[1] = (CO2Data >> 8) as u8;
                buf[2] = (CO2Data & 0xff) as u8;
                Ok(3)
            }
            SensorCommand::LastTemperatureResponse { Temperature } => {
                buf[0] = 0x08;
                buf[1] = (*Temperature as u16 >> 8) as u8;
                buf[2] = (*Temperature as u16 & 0xff) as u8;
                Ok(3)
            }
            SensorCommand::LastHumidityResponse { RelativeHumidity } => {
                buf[0] = 0x0a;
                buf[1] = (RelativeHumidity >> 8) as u8;
                buf[2] = (RelativeHumidity & 0xff) as u8;
                Ok(3)
            }
            _ => Err(()),
        }
    }
}
