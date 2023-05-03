#[derive(Clone, Debug)]
pub struct SetMeasurementInterval {
    pub measurement_interval: u16,
}

impl SetMeasurementInterval {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let measurement_interval = u16::from_be_bytes(buf);

        Self {
            measurement_interval,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(0_u8);
        out.push(self.measurement_interval.to_be_bytes());
        out
    }
}
#[derive(Clone, Debug)]
pub struct SetAltitude {
    pub altitude: u16,
}

impl SetAltitude {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let altitude = u16::from_be_bytes(buf);

        Self { altitude }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(1_u8);
        out.push(self.altitude.to_be_bytes());
        out
    }
}
#[derive(Clone, Debug)]
pub struct SetTemperatureOffset {
    pub temperature_offset: u16,
}

impl SetTemperatureOffset {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let temperature_offset = u16::from_be_bytes(buf);

        Self { temperature_offset }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(2_u8);
        out.push(self.temperature_offset.to_be_bytes());
        out
    }
}
#[derive(Clone, Debug)]
pub struct StartContinuousMeasurement {}

impl StartContinuousMeasurement {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(3_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct ReportNewData {}

impl ReportNewData {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(4_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct RequestLastCO2Data {}

impl RequestLastCO2Data {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(5_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct LastCO2DataResponse {
    pub co_2_data: u16,
}

impl LastCO2DataResponse {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let co_2_data = u16::from_be_bytes(buf);

        Self { co_2_data }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(6_u8);
        out.push(self.co_2_data.to_be_bytes());
        out
    }
}
#[derive(Clone, Debug)]
pub struct RequestLastTemperature {}

impl RequestLastTemperature {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(7_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct LastTemperatureResponse {
    pub temperature: i16,
}

impl LastTemperatureResponse {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let temperature = i16::from_be_bytes(buf);

        Self { temperature }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(8_u8);
        out.push(self.temperature.to_be_bytes());
        out
    }
}
#[derive(Clone, Debug)]
pub struct RequestLastHumidity {}

impl RequestLastHumidity {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(9_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct LastHumidityResponse {
    pub relative_humidity: u16,
}

impl LastHumidityResponse {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let relative_humidity = u16::from_be_bytes(buf);

        Self { relative_humidity }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(1_u8);
        out.push(10_u8);
        out.push(self.relative_humidity.to_be_bytes());
        out
    }
}
#[derive(Clone, Debug)]
pub struct Ping {}

impl Ping {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(222_u8);
        out.push(0_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct PingResponse {}

impl PingResponse {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(222_u8);
        out.push(1_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct EnableTestLed {}

impl EnableTestLed {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(170_u8);
        out.push(0_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct DisableTestLed {}

impl DisableTestLed {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(170_u8);
        out.push(1_u8);

        out
    }
}
#[derive(Clone, Debug)]
pub struct GenericResponse {
    pub command_number: u16,
    pub successful: bool,
}

impl GenericResponse {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let command_number = u16::from_be_bytes(buf);
        let successful = bool::from_be_bytes(buf);

        Self {
            command_number,
            successful,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out = vec![];
        out.push(170_u8);
        out.push(2_u8);
        out.push(self.command_number.to_be_bytes());
        out.push(self.successful.to_be_bytes());
        out
    }
}
