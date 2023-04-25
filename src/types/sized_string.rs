use std::{fmt::Display, hash::Hash};

use crate::{Error, NAME_MAX_SIZE, PATH_MAX_SIZE};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct SizedString<const T: usize>(String);

impl<const T: usize> SizedString<T> {
    pub fn try_from_str(s: &str) -> Result<Self, Error> {
        Self::try_from_slice(&s.as_bytes())
    }

    pub fn from_str_unchecked(s: &str) -> Self {
        Self::from_slice_unchecked(&s.as_bytes())
    }

    pub fn try_from_slice(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() > T {
            return Err(Error::SizedStringError);
        }
        match String::from_utf8(slice.to_owned()) {
            Ok(_) => Ok(Self::from_slice_unchecked(slice)),
            Err(_) => Err(Error::SizedStringError),
        }
    }

    pub fn from_slice_unchecked(slice: &[u8]) -> Self {
        let mut bytes: [u8; T] = [0; T];
        let len = slice.len();
        bytes[..len].copy_from_slice(&slice);
        let mut s = String::new();
        for i in 0..T {
            match String::from_utf8(vec![bytes[i]]) {
                Ok(char) => s = format!("{}{}", s, char),
                Err(_) => continue,
            };
        }
        Self(s)
    }

    pub fn to_string(self) -> String {
        self.0
    }

    pub fn to_bytes(self) -> [u8; T] {
        let mut bytes: [u8; T] = [0; T];
        bytes[..T].copy_from_slice(&self.0.as_bytes());
        bytes
    }
}

impl<const T: usize> Display for SizedString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

pub type Name = SizedString<NAME_MAX_SIZE>;

pub type Path = SizedString<PATH_MAX_SIZE>;
