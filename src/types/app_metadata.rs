use crate::constants::{NAME_MAX_SIZE, PATH_MAX_SIZE};
use crate::traits::{ToBytes, TryFromBytesList};
use crate::{Error, ToBytesList};

use super::{Name, Path, ServiceMetadata};

#[derive(Debug, PartialEq, Clone)]
pub struct AppMetadata {
    pub name: Name,
    pub path: Path,
    pub services: Vec<ServiceMetadata>,
}

impl Into<Vec<u8>> for AppMetadata {
    fn into(self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.name.to_bytes());
        data.extend_from_slice(&self.path.to_bytes());
        data.extend::<Vec<u8>>(self.services.to_bytes());
        data
    }
}

impl TryFrom<Vec<u8>> for AppMetadata {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let name = Name::try_from_slice(&value[pos..pos + NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        let path = Path::try_from_slice(&value[pos..pos + PATH_MAX_SIZE])?;
        pos += PATH_MAX_SIZE;
        let services = Vec::<ServiceMetadata>::try_from_bytes_list(&value[pos..])?;
        Ok(Self {
            name,
            path,
            services,
        })
    }
}

impl ToBytes for AppMetadata {}
