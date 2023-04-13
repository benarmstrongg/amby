use log::{error, info};
use std::io::Write;
use std::net::TcpStream;
use std::process;

use crate::traits::{ReadAll, ToBytesVec};
use crate::types::{Error, ReadRequest, Request, Result, WriteRequest};
use crate::{RegisterClient, Response};

pub struct Protocol {
    pub name: String,
    stream: TcpStream,
}

impl Protocol {
    pub fn create(name: &str) -> Self {
        Protocol::init_logger();
        let stream = Protocol::connect();
        Protocol::register_client(&stream, name.as_bytes());
        Self {
            name: name.into(),
            stream,
        }
    }

    fn init_logger() {
        match simple_logger::init() {
            Ok(_) => info!("Logger initialized"),
            Err(err) => println!("Error initializing logger. Error: {err}"),
        };
    }

    fn connect() -> TcpStream {
        match TcpStream::connect("127.0.0.1:4001") {
            Ok(stream) => {
                info!("Connected to tcp stream. Registering protocol");
                stream
            }
            Err(err) => {
                error!("Error connecting to tcp stream. Error: {err}");
                process::exit(1);
            }
        }
    }

    pub fn read(&mut self) -> Result<Vec<u8>> {
        match self.stream.read_all() {
            Ok(res) => Ok(res),
            Err(err) => return Err(Error::IoError(err)),
        }
    }

    pub fn send_read_request(
        &mut self,
        app_name: String,
        service_name: String,
        entity_name: String,
    ) -> Result<Response> {
        info!("Sending read request to app {}", &app_name);
        let req = Request::Read(ReadRequest {
            protocol_name: self.name.clone(),
            app_name,
            service_name,
            entity_name,
        });
        match self.stream.write_all(&req.to_bytes()) {
            Ok(_) => info!("Sent read request"),
            Err(err) => return Err(Error::IoError(err)),
        };
        match self.read() {
            Ok(res) => Ok(Response::from(res)),
            Err(err) => Err(err),
        }
    }

    pub fn send_write_request(
        &mut self,
        app_name: String,
        service_name: String,
        entity_name: String,
        data: &[u8],
    ) -> Result<Response> {
        info!("Sending write request to app {}", &app_name);
        let req = Request::Write(WriteRequest {
            protocol_name: self.name.clone(),
            app_name,
            service_name,
            entity_name,
            data: data.to_vec(),
        });
        match self.stream.write_all(&req.to_bytes()) {
            Ok(_) => info!("Sent write request"),
            Err(err) => return Err(Error::IoError(err)),
        };
        match self.read() {
            Ok(res) => Ok(Response::from(res)),
            Err(err) => Err(err),
        }
    }
}
