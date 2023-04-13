use crate::{Response, Result};

pub trait Entity {
    fn name(&self) -> String;
    fn read(&self) -> Result<Response>;
    fn write(&mut self, data: &[u8]) -> Result<Response>;

    fn path(&self) -> String {
        format!("/{}", self.name())
    }

    fn is_read(&self) -> bool {
        true
    }

    fn is_write(&self) -> bool {
        true
    }
}
