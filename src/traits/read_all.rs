use std::io::Read;

use crate::constants::BUFFER_STACK_SIZE;

pub trait ReadAll {
    fn read_all(&mut self) -> std::io::Result<Vec<u8>>;
}

impl<T> ReadAll for T
where
    T: Read,
{
    fn read_all(&mut self) -> std::io::Result<Vec<u8>> {
        let mut data = vec![];
        loop {
            let mut buf = [0; BUFFER_STACK_SIZE];
            let len = self.read(&mut buf)?;
            data.extend_from_slice(&buf[0..len]);
            if len < BUFFER_STACK_SIZE && len > 0 {
                return Ok(data);
            }
        }
    }
}
