use crate::{Error, ToBytesVec, BYTES_LIST_END, BYTES_LIST_SEPARATOR};

pub trait TryFromBytesList {
    fn try_from_bytes_list(bytes_list: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T: TryFrom<Vec<u8>, Error = Error> + Into<Vec<u8>> + ToBytesVec + Clone + std::fmt::Debug>
    TryFromBytesList for Vec<T>
{
    fn try_from_bytes_list(bytes_list: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        dbg!(String::from_utf8(bytes_list.to_vec()).unwrap());
        let mut obj_list = vec![];
        let mut start = 0;
        for pos in 0..bytes_list.len() {
            let byte = bytes_list[pos];
            if byte == BYTES_LIST_END {
                break;
            }
            if byte == BYTES_LIST_SEPARATOR {
                obj_list.push(T::try_from(bytes_list[start..=pos].to_vec())?);
                start = pos + 1;
            }
        }
        dbg!(&obj_list);
        dbg!(&obj_list.len());
        for obj in &obj_list {
            dbg!(String::from_utf8(obj.clone().to_bytes()).unwrap());
        }
        Ok(obj_list)
    }
}
