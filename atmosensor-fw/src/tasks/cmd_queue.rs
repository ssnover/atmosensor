use crate::{cmd_handlers, static_resources::CMD_QUEUE};

fn read_u16(buf: &[u8]) -> u16 {
    ((buf[0] as u16) << 8) | buf[1] as u16
}

#[derive(Copy, Clone)]
pub enum Command {
    Nop,
    Sensor(SensorCommand),
    Utility(UtilityCommand),
}

#[derive(Copy, Clone)]
pub enum SensorCommand {
    SetMeasurementInterval { MeasurementInterval: u16 },
}

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
            _ => None,
        }
    }

    pub fn to_bytes(&self, _buf: &mut [u8]) -> Result<usize, ()> {
        match self {
            _ => Err(()),
        }
    }
}

pub struct CommandQueue<const N: usize> {
    elements: [Command; N],
    write: usize,
    read: usize,
}

impl<const N: usize> CommandQueue<N> {
    pub fn new() -> Self {
        Self {
            elements: [Command::Nop; N],
            write: 0,
            read: 0,
        }
    }

    pub fn full(&self) -> bool {
        if self.read == 0 && self.write == N - 1 {
            true
        } else if self.write > self.read {
            false
        } else if self.read > self.write && self.write != self.read - 1 {
            false
        } else {
            false
        }
    }

    pub fn push(&mut self, cmd: Command) -> Result<(), ()> {
        if self.full() {
            Err(())
        } else {
            self.elements[self.write] = cmd;
            if self.write == N - 1 {
                self.write = 0;
            } else {
                self.write += 1;
            }
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<Command> {
        if self.write != self.read {
            let pop_idx = self.read;
            if self.read == N - 1 {
                self.read = 0;
            } else {
                self.read += 1;
            }
            Some(self.elements[pop_idx].clone())
        } else {
            None
        }
    }
}

pub struct CommandHandler {}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler {}
    }

    pub fn run(&self) {
        if let Some(cmd) = unsafe { CMD_QUEUE.assume_init_mut().pop() } {
            match cmd {
                Command::Nop => {}
                Command::Utility(UtilityCommand::EnableTestLed) => {
                    cmd_handlers::led::enable_test_led();
                }
                Command::Utility(UtilityCommand::DisableTestLed) => {
                    cmd_handlers::led::disable_test_led();
                }
                Command::Sensor(SensorCommand::SetMeasurementInterval {
                    MeasurementInterval,
                }) => {
                    cmd_handlers::sensor::set_measurement_interval(MeasurementInterval);
                }
                _ => {}
            }
        }
    }
}
