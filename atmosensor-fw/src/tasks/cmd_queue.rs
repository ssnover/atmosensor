use crate::cmd_handlers;
use crate::tasks::{Command, SensorCommand, UtilityCommand};

static mut CMD_QUEUE: CommandQueue<48> = CommandQueue::new();

pub fn push_new_cmd(cmd: &Command) {
    let _ = critical_section::with(|_cs| unsafe { CMD_QUEUE.push(*cmd) });
}

pub struct CommandQueue<const N: usize> {
    elements: [Command; N],
    write: usize,
    read: usize,
}

impl<const N: usize> CommandQueue<N> {
    pub const fn new() -> Self {
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
        let cmd = critical_section::with(|_cs| unsafe { CMD_QUEUE.pop() });
        if let Some(cmd) = cmd {
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
                Command::Sensor(SensorCommand::SetAltitude { Altitude }) => {
                    cmd_handlers::sensor::set_altitude(Altitude);
                }
                Command::Sensor(SensorCommand::SetTemperatureOffset { TemperatureOffset }) => {
                    cmd_handlers::sensor::set_temperature_offset(TemperatureOffset);
                }
                Command::Sensor(SensorCommand::StartContinuousMeasurement) => {
                    cmd_handlers::sensor::start_continuous_measurement();
                }
                Command::Sensor(SensorCommand::ReportNewData) => {
                    cmd_handlers::sensor::handle_data_ready();
                }
                Command::Sensor(SensorCommand::RequestLastCO2Data) => {
                    cmd_handlers::sensor::handle_request_co2_data();
                }
                _ => {}
            }
        }
    }
}
