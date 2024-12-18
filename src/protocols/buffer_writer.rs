use core::fmt::{self, Write};


pub struct BufferWriter<'a> {
    buffer: &'a mut [u8],
    position: usize,
}

impl<'a> BufferWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, position: 0 }
    }

    pub fn written_data(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.position]).unwrap()
    }
}

// Implémente Write pour le buffer
impl<'a> Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining_space = self.buffer.len() - self.position;

        if bytes.len() > remaining_space {
            return Err(fmt::Error);
        }

        self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
        self.position += bytes.len();

        Ok(())
    }
}

