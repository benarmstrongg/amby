use crate::constants::{NAME_MAX_SIZE, REQUEST_TYPE_SIZE};
use crate::traits::{ToBytes, ToSlice, TryFromSlice};
use crate::types::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    Read(ReadRequest),
    Write(WriteRequest),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReadRequest {
    pub protocol_name: String,
    pub app_name: String,
    pub service_name: String,
    pub entity_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WriteRequest {
    pub protocol_name: String,
    pub app_name: String,
    pub service_name: String,
    pub entity_name: String,
    pub data: Vec<u8>,
}

impl Into<Vec<u8>> for Request {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(match &self {
            Self::Read(_) => &[0],
            Self::Write(_) => &[1],
        });
        let protocol_name = match &self {
            Self::Read(ReadRequest { protocol_name, .. })
            | Self::Write(WriteRequest { protocol_name, .. }) => protocol_name.clone(),
        };
        bytes.extend_from_slice(&protocol_name.to_slice::<NAME_MAX_SIZE>());
        let app_name = match &self {
            Self::Read(ReadRequest { app_name, .. })
            | Self::Write(WriteRequest { app_name, .. }) => app_name.clone(),
        };
        bytes.extend_from_slice(&app_name.to_slice::<NAME_MAX_SIZE>());
        let service_name = match &self {
            Self::Read(ReadRequest { service_name, .. })
            | Self::Write(WriteRequest { service_name, .. }) => service_name.clone(),
        };
        bytes.extend_from_slice(&service_name.to_slice::<NAME_MAX_SIZE>());
        let entity_name = match &self {
            Self::Read(ReadRequest { entity_name, .. })
            | Self::Write(WriteRequest { entity_name, .. }) => entity_name.clone(),
        };
        bytes.extend_from_slice(&entity_name.to_slice::<NAME_MAX_SIZE>());
        match self {
            Self::Read(_) => bytes,
            Self::Write(WriteRequest { data, .. }) => {
                bytes.extend(data);
                bytes
            }
        }
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let request_type = value[0];
        let mut pos = REQUEST_TYPE_SIZE;
        let protocol_name = String::try_from_slice(&value[pos..pos + NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        let app_name = String::try_from_slice(&value[pos..pos + NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        let service_name = String::try_from_slice(&value[pos..pos + NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        let entity_name = String::try_from_slice(&value[pos..pos + NAME_MAX_SIZE])?;
        pos += NAME_MAX_SIZE;
        if request_type == 0 {
            Ok(Self::Read(ReadRequest {
                protocol_name,
                app_name,
                service_name,
                entity_name,
            }))
        } else {
            let data = value[pos..].to_vec();
            Ok(Self::Write(WriteRequest {
                protocol_name,
                app_name,
                service_name,
                entity_name,
                data,
            }))
        }
    }
}

impl ToBytes for Request {}
