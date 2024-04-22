use std::io::{Read, Write};

pub struct MockInputOutput {
    input: Vec<&'static str>,
    output: Vec<u8>,
    input_index: usize,
}

impl MockInputOutput {
    pub fn new(input: Vec<&'static str>) -> Self {
        MockInputOutput {
            input,
            output: Vec::new(),
            input_index: 0,
        }
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }
}

impl Read for MockInputOutput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.input_index < self.input.len() {
            let input_str = self.input[self.input_index];
            self.input_index += 1;
            let bytes = input_str.as_bytes();
            let len = std::cmp::min(bytes.len(), buf.len());
            buf[..len].copy_from_slice(&bytes[..len]);
            Ok(len)
        } else {
            Ok(0)
        }
    }
}

impl Write for MockInputOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}