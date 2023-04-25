use log::{error, info};
use std::io::Write;
use std::net::TcpStream;
use std::process;
use std::time::Duration;

use crate::traits::{ReadAll, RegisterClient, ToBytes};
use crate::types::{Error, ReadRequest, Request, Result, WriteRequest};
use crate::{Name, Response};

use super::AppRegistration;

pub struct Protocol {
    pub name: Name,
    stream: TcpStream,
}

impl Protocol {
    pub fn create(name: Name) -> Self {
        Protocol::init_logger();
        let stream = Protocol::connect_protocol_stream();
        Protocol::register_client(&stream, &name.clone().to_bytes());
        Self { name, stream }
    }

    fn init_logger() {
        match simple_logger::init() {
            Ok(()) => info!("Logger initialized"),
            Err(err) => println!("Error initializing logger. Error: {err}"),
        };
    }

    fn connect_protocol_stream() -> TcpStream {
        match TcpStream::connect("127.0.0.1:4001") {
            Ok(stream) => {
                info!("Connected to protocol tcp stream. Registering protocol");
                stream
            }
            Err(err) => {
                error!("Error connecting to protocol tcp stream. Error: {err}");
                process::exit(1);
            }
        }
    }

    fn connect_app_registration_stream() -> TcpStream {
        match TcpStream::connect("127.0.0.1:4002") {
            Ok(stream) => {
                info!("Connected to app registration tcp stream");
                stream
            }
            Err(err) => {
                error!("Error connecting to app registration tcp stream. Error: {err}");
                process::exit(1);
            }
        }
    }

    pub fn on_app_register(&self) -> AppRegistration {
        let app_registration_stream = Protocol::connect_app_registration_stream();
        app_registration_stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        AppRegistration {
            stream: app_registration_stream,
        }
    }

    pub fn read(&mut self) -> Result<Vec<u8>> {
        match self.stream.read_all() {
            Ok(res) => Ok(res),
            Err(err) => return Err(Error::IoError(err)),
        }
    }

    pub fn send_read_request(&mut self, req: ReadRequest) -> Result<Response> {
        info!("Sending read request to app {}", &req.app_name);
        let requset = Request::Read(req);
        match self.stream.write_all(&requset.to_bytes()) {
            Ok(_) => info!("Sent read request"),
            Err(err) => return Err(Error::IoError(err)),
        };
        match self.read() {
            Ok(res) => Ok(Response::from(res)),
            Err(err) => Err(err),
        }
    }

    pub fn send_write_request(&mut self, req: WriteRequest) -> Result<Response> {
        info!("Sending write request to app {}", &req.app_name);
        let request = Request::Write(req);
        match self.stream.write_all(&request.to_bytes()) {
            Ok(_) => info!("Sent write request"),
            Err(err) => return Err(Error::IoError(err)),
        };
        match self.read() {
            Ok(res) => Ok(Response::from(res)),
            Err(err) => Err(err),
        }
    }
}

impl Clone for Protocol {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            stream: self.stream.try_clone().unwrap(),
        }
    }
}
