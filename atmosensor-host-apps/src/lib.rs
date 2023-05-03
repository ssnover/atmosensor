use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{ReadHalf, WriteHalf};
use tokio_serial::SerialStream;

pub mod mock;
pub mod protocol;

pub struct Atmosensor {
    writer: Writer,
    reader: Reader,
    encoded_tx_buffer: [u8; 1024],
}

impl Atmosensor {
    pub fn new(serial_port: tokio_serial::SerialStream) -> Self {
        let (read_stream, write_stream) = tokio::io::split(serial_port);
        Self {
            writer: Writer { write_stream },
            reader: Reader { read_stream },
            encoded_tx_buffer: [0u8; 1024],
        }
    }

    pub async fn send_bytes(&mut self, data: &[u8]) -> std::io::Result<()> {
        let bytes_encoded = cobs::encode(data, &mut self.encoded_tx_buffer);
        self.encoded_tx_buffer[bytes_encoded] = 0x00;

        self.writer
            .send_bytes(&self.encoded_tx_buffer[..=bytes_encoded])
            .await
    }
}

struct Reader {
    read_stream: ReadHalf<SerialStream>,
}

struct Writer {
    write_stream: WriteHalf<SerialStream>,
}

impl Writer {
    pub async fn send_bytes(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.write_stream.write_all(data).await
    }
}
