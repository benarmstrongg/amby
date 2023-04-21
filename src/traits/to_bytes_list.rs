use crate::{ToBytesVec, BYTES_LIST_END, BYTES_LIST_SEPARATOR};

pub trait ToBytesList {
    fn to_bytes_list<T: Into<Vec<u8>> + ToBytesVec>(self) -> Vec<u8>
    where
        Self: IntoIterator<Item = T> + Sized,
    {
        let mut bytes = vec![];
        for value in self {
            bytes.extend_from_slice(&value.to_bytes());
            bytes.push(BYTES_LIST_SEPARATOR);
        }
        bytes.push(BYTES_LIST_END);
        bytes
    }
}

impl<T: Into<Vec<u8>> + ToBytesVec> ToBytesList for Vec<T> {}
