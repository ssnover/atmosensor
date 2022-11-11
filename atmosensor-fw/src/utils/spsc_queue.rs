use bare_metal::CriticalSection;
use core::cmp::Ordering;

pub struct SpscQueue<'a, T, const N: usize> {
    data: &'a mut [T; N],
    write: usize,
    read: usize,
}

impl<'a, T, const N: usize> SpscQueue<'a, T, N>
where
    T: 'a + Copy,
{
    pub fn new(buf: &'a mut [T; N]) -> SpscQueue<'a, T, N> {
        SpscQueue {
            data: buf,
            write: 0,
            read: 0,
        }
    }

    pub const fn len() -> usize {
        N
    }

    pub fn available_to_read<'cs>(&'cs self, _cs: CriticalSection<'cs>) -> usize {
        match self.read.cmp(&self.write) {
            Ordering::Equal => 0,
            Ordering::Greater => {
                N - self.read + self.write
            },
            Ordering::Less => {
                self.write - self.read
            }
        }
    }

    pub fn read<'cs>(&'cs mut self, _cs: CriticalSection<'cs>, buf: &mut [T]) -> usize {
        match self.read.cmp(&self.write) {
            Ordering::Equal => 0,
            Ordering::Greater => {
                // Read through the end of the buffer and then wrap around up to write pointer
                let available_to_end = N - self.read;
                let available_for_read = available_to_end + self.write;
                let mut bytes_read = 0;
                for read_idx in self.read..core::cmp::min(N, buf.len()) {
                    // Read to the end of the memory buffer first
                    buf[bytes_read] = self.data[read_idx];
                    bytes_read += 1;
                    self.read += 1;
                }
                if self.read == N {
                    self.read = 0;
                }
                if bytes_read < buf.len() {
                    for read_idx in 0..core::cmp::min(self.write, buf.len() - bytes_read) {
                        buf[bytes_read] = self.data[read_idx];
                        bytes_read += 1;
                        self.read += 1;
                    }
                }
                bytes_read
            },
            Ordering::Less => {
                let available_for_read = self.write - self.read;
                let mut bytes_read = 0;
                for read_idx in self.read..core::cmp::min(self.write, self.read + buf.len()) {
                    buf[bytes_read] = self.data[read_idx];
                    bytes_read += 1;
                    self.read += 1;
                }
                bytes_read
            }
        }
    }

    pub fn write<'cs>(&'cs mut self, _cs: CriticalSection<'cs>, buf: &[T]) -> usize {
        let available_without_overwrite = match self.write.cmp(&self.read) {
            Ordering::Equal => {
                // Whole buffer is available to write to
                N
            },
            Ordering::Greater => {
                // Can write to end of buffer and then some before overwrite
                N - self.write + self.read
            },
            Ordering::Less => {
                // Can
                self.read - self.write
            }
        };
        let mut bytes_written = 0;
        if (N - self.write) > buf.len() {
            for write_idx in 0..buf.len() {
                self.data[self.write] = buf[write_idx];
                self.write += 1;
                bytes_written += 1;
            }
        } else {
            for write_idx in 0..(N - self.write) {
                self.data[self.write] = buf[write_idx];
                self.write += 1;
                bytes_written += 1;
            }
            assert_eq!(self.write, N);
            self.write = 0;
            let bytes_remaining = buf.len() - bytes_written;
            for write_idx in bytes_written..buf.len() {
                self.data[self.write] = buf[write_idx];
                self.write += 1;
                bytes_written += 1;
            }
        }
        if available_without_overwrite < buf.len() {
            self.read = self.write + 1;
            if self.read == N {
                self.read = 0;
            }
        }
        bytes_written
    }
}
