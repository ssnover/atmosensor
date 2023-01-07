use crate::cmd_handlers;

#[derive(Copy, Clone)]
pub enum Command {
    Nop,
    Utility(UtilityCommand),
}

#[derive(Copy, Clone)]
pub enum UtilityCommand {
    EnableTestLed,
    DisableTestLed,
}

impl Command {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            0xaa => {
                Some(Command::Utility(UtilityCommand::from_bytes(&buf[1..])?))
            },
            _ => {
                None
            }
        }
    }
}

impl UtilityCommand {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            0x00 => {
                Some(UtilityCommand::EnableTestLed)
            },
            0x01 => {
                Some(UtilityCommand::DisableTestLed)
            }
            _ => {
                None
            }
        }
    }
}

pub struct CommandQueue<'a, const N: usize> {
    elements: &'a mut [Command; N],
    write: usize,
    read: usize,
}

impl<'a, const N: usize> CommandQueue<'a, N> {
    pub fn new(buf: &'a mut [Command; N]) -> Self {
        Self {
            elements: buf,
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

pub fn command_handler_run<'a, const N: usize>(cmd_queue: &mut CommandQueue<'a, N>) {
    if let Some(cmd) = cmd_queue.pop() {
        match cmd {
            Command::Nop => {}
            Command::Utility(UtilityCommand::EnableTestLed) => {
                cmd_handlers::led::enable_test_led();
            }
            Command::Utility(UtilityCommand::DisableTestLed) => {
                cmd_handlers::led::disable_test_led();
            }
        }
    }
}
