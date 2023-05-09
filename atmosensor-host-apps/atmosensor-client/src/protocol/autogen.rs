#![allow(unused_mut)]

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Clone, Debug)]
pub enum Command {
    SetMeasurementInterval(SetMeasurementInterval),
    SetAltitude(SetAltitude),
    SetTemperatureOffset(SetTemperatureOffset),
    StartContinuousMeasurement(StartContinuousMeasurement),
    ReportNewData(ReportNewData),
    RequestLastCO2Data(RequestLastCO2Data),
    LastCO2DataResponse(LastCO2DataResponse),
    RequestLastTemperature(RequestLastTemperature),
    LastTemperatureResponse(LastTemperatureResponse),
    RequestLastHumidity(RequestLastHumidity),
    LastHumidityResponse(LastHumidityResponse),
    Ping(Ping),
    PingResponse(PingResponse),
    EnableTestLed(EnableTestLed),
    DisableTestLed(DisableTestLed),
    GenericResponse(GenericResponse),
}

impl Command {
    pub fn from_bytes(buf: &[u8]) -> Self {
        match (buf[0], buf[1]) {
            (1, 0) => {
                Command::SetMeasurementInterval(SetMeasurementInterval::from_bytes(&buf[2..]))
            }
            (1, 1) => Command::SetAltitude(SetAltitude::from_bytes(&buf[2..])),
            (1, 2) => Command::SetTemperatureOffset(SetTemperatureOffset::from_bytes(&buf[2..])),
            (1, 3) => Command::StartContinuousMeasurement(StartContinuousMeasurement::from_bytes(
                &buf[2..],
            )),
            (1, 4) => Command::ReportNewData(ReportNewData::from_bytes(&buf[2..])),
            (1, 5) => Command::RequestLastCO2Data(RequestLastCO2Data::from_bytes(&buf[2..])),
            (1, 6) => Command::LastCO2DataResponse(LastCO2DataResponse::from_bytes(&buf[2..])),
            (1, 7) => {
                Command::RequestLastTemperature(RequestLastTemperature::from_bytes(&buf[2..]))
            }
            (1, 8) => {
                Command::LastTemperatureResponse(LastTemperatureResponse::from_bytes(&buf[2..]))
            }
            (1, 9) => Command::RequestLastHumidity(RequestLastHumidity::from_bytes(&buf[2..])),
            (1, 10) => Command::LastHumidityResponse(LastHumidityResponse::from_bytes(&buf[2..])),
            (222, 0) => Command::Ping(Ping::from_bytes(&buf[2..])),
            (222, 1) => Command::PingResponse(PingResponse::from_bytes(&buf[2..])),
            (170, 0) => Command::EnableTestLed(EnableTestLed::from_bytes(&buf[2..])),
            (170, 1) => Command::DisableTestLed(DisableTestLed::from_bytes(&buf[2..])),
            (170, 2) => Command::GenericResponse(GenericResponse::from_bytes(&buf[2..])),
            _ => unimplemented!(),
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Command::SetMeasurementInterval(inner) => inner.to_bytes(),
            Command::SetAltitude(inner) => inner.to_bytes(),
            Command::SetTemperatureOffset(inner) => inner.to_bytes(),
            Command::StartContinuousMeasurement(inner) => inner.to_bytes(),
            Command::ReportNewData(inner) => inner.to_bytes(),
            Command::RequestLastCO2Data(inner) => inner.to_bytes(),
            Command::LastCO2DataResponse(inner) => inner.to_bytes(),
            Command::RequestLastTemperature(inner) => inner.to_bytes(),
            Command::LastTemperatureResponse(inner) => inner.to_bytes(),
            Command::RequestLastHumidity(inner) => inner.to_bytes(),
            Command::LastHumidityResponse(inner) => inner.to_bytes(),
            Command::Ping(inner) => inner.to_bytes(),
            Command::PingResponse(inner) => inner.to_bytes(),
            Command::EnableTestLed(inner) => inner.to_bytes(),
            Command::DisableTestLed(inner) => inner.to_bytes(),
            Command::GenericResponse(inner) => inner.to_bytes(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SetMeasurementInterval {
    pub measurement_interval: u16,
}

impl SetMeasurementInterval {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        let measurement_interval = cursor.read_u16::<BigEndian>().unwrap();

        Self {
            measurement_interval,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 0_u8];

        out.extend_from_slice(&self.measurement_interval.to_be_bytes());
        out
    }
}

#[derive(Clone, Debug)]
pub struct SetAltitude {
    pub altitude: u16,
}

impl SetAltitude {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        let altitude = cursor.read_u16::<BigEndian>().unwrap();

        Self { altitude }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 1_u8];

        out.extend_from_slice(&self.altitude.to_be_bytes());
        out
    }
}

#[derive(Clone, Debug)]
pub struct SetTemperatureOffset {
    pub temperature_offset: u16,
}

impl SetTemperatureOffset {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        let temperature_offset = cursor.read_u16::<BigEndian>().unwrap();

        Self { temperature_offset }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 2_u8];

        out.extend_from_slice(&self.temperature_offset.to_be_bytes());
        out
    }
}

#[derive(Clone, Debug)]
pub struct StartContinuousMeasurement {}

impl StartContinuousMeasurement {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 3_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct ReportNewData {}

impl ReportNewData {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 4_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct RequestLastCO2Data {}

impl RequestLastCO2Data {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 5_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct LastCO2DataResponse {
    pub co_2_data: u16,
}

impl LastCO2DataResponse {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        let co_2_data = cursor.read_u16::<BigEndian>().unwrap();

        Self { co_2_data }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 6_u8];

        out.extend_from_slice(&self.co_2_data.to_be_bytes());
        out
    }
}

#[derive(Clone, Debug)]
pub struct RequestLastTemperature {}

impl RequestLastTemperature {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 7_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct LastTemperatureResponse {
    pub temperature: i16,
}

impl LastTemperatureResponse {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        let temperature = cursor.read_i16::<BigEndian>().unwrap();

        Self { temperature }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 8_u8];

        out.extend_from_slice(&self.temperature.to_be_bytes());
        out
    }
}

#[derive(Clone, Debug)]
pub struct RequestLastHumidity {}

impl RequestLastHumidity {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 9_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct LastHumidityResponse {
    pub relative_humidity: u16,
}

impl LastHumidityResponse {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        let relative_humidity = cursor.read_u16::<BigEndian>().unwrap();

        Self { relative_humidity }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![1_u8, 10_u8];

        out.extend_from_slice(&self.relative_humidity.to_be_bytes());
        out
    }
}

#[derive(Clone, Debug)]
pub struct Ping {}

impl Ping {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![222_u8, 0_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct PingResponse {}

impl PingResponse {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![222_u8, 1_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct EnableTestLed {}

impl EnableTestLed {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![170_u8, 0_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct DisableTestLed {}

impl DisableTestLed {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![170_u8, 1_u8];

        out
    }
}

#[derive(Clone, Debug)]
pub struct GenericResponse {
    pub successful: bool,
}

impl GenericResponse {
    #[allow(unused)]
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        let successful = cursor.read_u8().unwrap() != 0;

        Self { successful }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![170_u8, 2_u8];

        out.push(self.successful as u8);

        out
    }
}
