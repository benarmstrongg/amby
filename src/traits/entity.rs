use crate::{Name, Path, Response, Result};

pub trait Entity {
    fn name(&self) -> Name;
    fn read(&self) -> Result<Response>;
    fn write(&mut self, data: &[u8]) -> Result<Response>;

    fn path(&self) -> Path {
        Path::from_str_unchecked(&format!("/{}", self.name()))
    }

    fn is_read(&self) -> bool {
        true
    }

    fn is_write(&self) -> bool {
        true
    }
}
