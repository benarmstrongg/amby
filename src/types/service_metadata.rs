use crate::constants::{NAME_MAX_SIZE, PATH_MAX_SIZE};
use crate::traits::{ToBytes, ToBytesList, TryFromBytesList};

use super::{EntityMetadata, Error, Name, Path};

#[derive(Debug, PartialEq, Clone)]
pub struct ServiceMetadata {
    pub name: Name,
    pub path: Path,
    pub entities: Vec<EntityMetadata>,
}

impl Into<Vec<u8>> for ServiceMetadata {
    fn into(self) -> Vec<u8> {
        let mut data = vec![];
        data.extend(&self.name.to_bytes());
        data.extend(&self.path.to_bytes());
        data.extend(self.entities.to_bytes());
        data
    }
}

impl TryFrom<Vec<u8>> for ServiceMetadata {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let name = Name::try_from_slice(&value[pos..NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        let path = Path::try_from_slice(&value[pos..pos + PATH_MAX_SIZE])?;
        pos += PATH_MAX_SIZE;
        let entities = Vec::<EntityMetadata>::try_from_bytes_list(&value[pos..])?;
        Ok(Self {
            name,
            path,
            entities,
        })
    }
}

impl ToBytes for ServiceMetadata {}
