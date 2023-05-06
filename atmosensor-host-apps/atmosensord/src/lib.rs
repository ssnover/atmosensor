use protocol::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{ReadHalf, WriteHalf};
use tokio_serial::SerialStream;

pub mod mock;
pub mod protocol;

pub struct Atmosensor {
    writer: Writer,
    reader: Reader,
}

impl Atmosensor {
    pub fn new(serial_port: tokio_serial::SerialStream) -> Self {
        let (read_stream, write_stream) = tokio::io::split(serial_port);
        Self {
            writer: Writer::new(write_stream),
            reader: Reader::new(read_stream),
        }
    }

    pub fn split(self) -> (Reader, Writer) {
        (self.reader, self.writer)
    }

    pub async fn send(&mut self, cmd: Command) -> std::io::Result<()> {
        self.writer.send(cmd).await
    }

    pub async fn receive_next(&mut self, timeout: std::time::Duration) -> Option<Command> {
        self.reader.receive_next(timeout).await
    }
}

pub struct Reader {
    read_stream: ReadHalf<SerialStream>,
    encoded_rx_buffer: Box<[u8; 1024]>,
    decoded_rx_buffer: Box<[u8; 1024]>,
}

impl Reader {
    fn new(stream: ReadHalf<SerialStream>) -> Self {
        Self {
            read_stream: stream,
            encoded_rx_buffer: Box::new([0u8; 1024]),
            decoded_rx_buffer: Box::new([0u8; 1024]),
        }
    }

    pub async fn receive_next(&mut self, timeout: std::time::Duration) -> Option<Command> {
        match tokio::time::timeout(timeout, self.receive()).await {
            Ok(cmd) => Some(cmd),
            Err(_) => None,
        }
    }

    pub async fn receive(&mut self) -> Command {
        loop {
            if let Ok(bytes_read) = self.read_stream.read(&mut *self.encoded_rx_buffer).await {
                if let Ok(bytes_decoded) = cobs::decode(
                    &self.encoded_rx_buffer[..bytes_read],
                    &mut *self.decoded_rx_buffer,
                ) {
                    // todo: Revisit failure cases here
                    break Command::from_bytes(&self.decoded_rx_buffer[..bytes_decoded]);
                }
            }
        }
    }

    pub async fn receive_raw(&mut self) -> Vec<u8> {
        loop {
            if let Ok(bytes_read) = self.read_stream.read(&mut *self.encoded_rx_buffer).await {
                if let Ok(bytes_decoded) = cobs::decode(
                    &self.encoded_rx_buffer[..bytes_read],
                    &mut *self.decoded_rx_buffer,
                ) {
                    break Vec::from(&self.decoded_rx_buffer[..bytes_decoded]);
                }
            }
        }
    }
}

pub struct Writer {
    write_stream: WriteHalf<SerialStream>,
    encoded_tx_buffer: Box<[u8; 1024]>,
}

impl Writer {
    fn new(stream: WriteHalf<SerialStream>) -> Self {
        Self {
            write_stream: stream,
            encoded_tx_buffer: Box::new([0u8; 1024]),
        }
    }

    pub async fn send(&mut self, cmd: Command) -> std::io::Result<()> {
        self.send_raw(&cmd.to_bytes()).await
    }

    pub async fn send_raw(&mut self, data: &[u8]) -> std::io::Result<()> {
        let bytes_encoded = cobs::encode(data, &mut *self.encoded_tx_buffer);
        self.encoded_tx_buffer[bytes_encoded] = 0x00;
        self.write_stream
            .write_all(&self.encoded_tx_buffer[..=bytes_encoded])
            .await
    }
}
