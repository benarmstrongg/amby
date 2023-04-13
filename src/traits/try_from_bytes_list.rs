use crate::{Error, BYTES_LIST_SEPARATOR};

pub trait TryFromBytesList {
    fn try_from_bytes_list(bytes_list: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T: TryFrom<Vec<u8>, Error = Error>> TryFromBytesList for Vec<T> {
    fn try_from_bytes_list(bytes_list: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut obj_list = vec![];
        let mut start = 0;
        for pos in 0..bytes_list.len() {
            if pos <= start {
                continue;
            }
            let byte = bytes_list[pos];
            if byte == BYTES_LIST_SEPARATOR {
                obj_list.push(T::try_from(bytes_list[start..(pos - 1)].to_vec())?);
                start = pos + 1;
            }
        }
        Ok(obj_list)
    }
}
