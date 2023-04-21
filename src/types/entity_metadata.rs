use crate::constants::{NAME_MAX_SIZE, PATH_MAX_SIZE};
use crate::traits::{ToBytes, ToSlice, TryFromSlice};
use crate::types::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct EntityMetadata {
    pub name: String,
    pub path: String,
    pub read: bool,
    pub write: bool,
}

impl Into<Vec<u8>> for EntityMetadata {
    fn into(self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.name.to_slice::<NAME_MAX_SIZE>());
        data.extend_from_slice(&self.path.to_slice::<PATH_MAX_SIZE>());
        data.push(self.read.into());
        data.push(self.write.into());
        data
    }
}

impl TryFrom<Vec<u8>> for EntityMetadata {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let name = String::try_from_slice(&value[0..NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        let path = String::try_from_slice(&value[pos..pos + PATH_MAX_SIZE])?;
        pos += PATH_MAX_SIZE;
        let read = value[pos] == 1;
        pos += 1;
        let write = value[pos] == 1;
        Ok(Self {
            name,
            path,
            read,
            write,
        })
    }
}

impl ToBytes for EntityMetadata {}
