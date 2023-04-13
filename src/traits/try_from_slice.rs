use crate::Error;

pub trait TryFromSlice {
    fn try_from_slice(value: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}

impl TryFromSlice for String {
    fn try_from_slice(value: &[u8]) -> Result<Self, Error> {
        match String::from_utf8(value.to_vec()) {
            Ok(name) => Ok(name.trim_matches(char::from(0)).to_string()),
            Err(err) => return Err(Error::StringParseError(err)),
        }
    }
}
