use crate::{ToBytesVec, RESPONSE_TYPE_SIZE};

#[derive(Debug, PartialEq, Clone)]
pub enum Response {
    Success(Vec<u8>),
    Error(Vec<u8>),
    Empty,
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        match self {
            Response::Success(res) => {
                bytes.extend_from_slice(&[0]);
                bytes.extend(res);
            }
            Response::Error(err) => {
                bytes.extend_from_slice(&[1]);
                bytes.extend(err);
            }
            Response::Empty => {
                return vec![];
            }
        };
        bytes
    }
}

impl From<Vec<u8>> for Response {
    fn from(value: Vec<u8>) -> Self {
        let request_type = match value.get(0) {
            Some(byte) => byte,
            None => return Self::Empty,
        };
        let value = value[RESPONSE_TYPE_SIZE..].to_vec();
        match request_type {
            0 => Self::Success(value),
            1 => Self::Error(value),
            _ => Self::Empty,
        }
    }
}

impl ToBytesVec for Response {}
