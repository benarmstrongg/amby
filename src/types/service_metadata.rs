use crate::constants::{NAME_MAX_SIZE, PATH_MAX_SIZE};
use crate::traits::{ToBytesList, ToBytesSlice, ToBytesVec, TryFromBytesList, TryFromSlice};

use super::{EntityMetadata, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct ServiceMetadata {
    pub name: String,
    pub path: String,
    pub entities: Vec<EntityMetadata>,
}

impl Into<Vec<u8>> for ServiceMetadata {
    fn into(self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.name.to_slice::<NAME_MAX_SIZE>());
        data.extend_from_slice(&self.path.to_slice::<PATH_MAX_SIZE>());
        data.extend(self.entities.to_bytes_list());
        data
    }
}

impl TryFrom<Vec<u8>> for ServiceMetadata {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let name = String::try_from_slice(&value[pos..NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        let path = String::try_from_slice(&value[pos..pos + PATH_MAX_SIZE])?;
        pos += PATH_MAX_SIZE;
        let entities = Vec::<EntityMetadata>::try_from_bytes_list(&value[pos..])?;
        Ok(Self {
            name,
            path,
            entities,
        })
    }
}

impl ToBytesVec for ServiceMetadata {}
