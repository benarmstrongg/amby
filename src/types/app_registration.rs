use std::net::TcpStream;

use log::info;

use crate::{traits::ReadAll, AppMetadata, Error};

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct AppRegistration {
    pub stream: TcpStream,
}

impl Iterator for AppRegistration {
    type Item = Result<AppMetadata, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.read_all() {
            Ok(res) => match AppMetadata::try_from(res.clone()) {
                Ok(metadata) => {
                    info!("App {} registered", &metadata.name);
                    Some(Ok(metadata))
                }
                Err(err) => Some(Err(err)),
            },
            Err(err) => Some(Err(Error::IoError(err))),
        }
    }
}
