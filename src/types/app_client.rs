use log::{error, info};
use std::process;
use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::traits::{Entity, ReadAll, RegisterClient, Service, ToBytes};
use crate::types::{AppMetadata, EntityMetadata, Request, ServiceMetadata};
use crate::{Name, Path, ReadRequest, Response, WriteRequest};

pub struct App {
    stream: TcpStream,
    metadata: AppMetadata,
    entities: HashMap<Name, Box<dyn Entity + 'static>>,
}

impl App {
    pub fn create(name: Name) -> Self {
        App::init_logger();
        let path = Path::from_str_unchecked(&format!("/{}", &name));
        let metadata = AppMetadata {
            name,
            path,
            services: vec![],
        };
        let stream = App::connect();
        Self {
            stream,
            metadata,
            entities: HashMap::new(),
        }
    }

    fn init_logger() {
        match simple_logger::init() {
            Ok(_) => info!("Logger initialized"),
            Err(err) => println!("Error initializing logger. Error: {err}"),
        };
    }

    fn connect() -> TcpStream {
        match TcpStream::connect("127.0.0.1:4000") {
            Ok(stream) => {
                info!("Connected to tcp stream. Registering app");
                stream
            }
            Err(err) => {
                error!("Error connecting to tcp stream. Error: {err}");
                process::exit(1);
            }
        }
    }

    pub fn path(mut self, path: Path) -> Self {
        self.metadata.path = path;
        self
    }

    pub fn service(mut self, service: impl Service) -> Self {
        let mut entity_metadata: Vec<EntityMetadata> = vec![];
        let entities = service.get_entities();
        for entity in entities {
            let path = Path::from_str_unchecked(&format!(
                "{}{}{}",
                &self.metadata.path,
                service.path(),
                entity.path()
            ));
            entity_metadata.push(EntityMetadata {
                name: entity.name(),
                read: entity.is_read(),
                write: entity.is_write(),
                path,
            });
            self.entities.insert(entity.name(), entity);
        }
        let path = Path::from_str_unchecked(&format!("{}{}", &self.metadata.path, service.path()));
        let service_metadata = ServiceMetadata {
            name: service.name(),
            entities: entity_metadata,
            path,
        };
        self.metadata.services.push(service_metadata);
        self
    }

    pub fn serve(&mut self) {
        App::register_client(&self.stream, &self.metadata.clone().to_bytes());
        loop {
            let req = match self.pool_request() {
                Ok(req) => req,
                Err(()) => continue,
            };
            match req {
                Request::Read(req) => self.handle_read(req),
                Request::Write(req) => self.handle_write(req),
            };
        }
    }

    fn pool_request(&mut self) -> Result<Request, ()> {
        match self.stream.read_all() {
            Ok(bytes) => match Request::try_from(bytes) {
                Ok(req) => Ok(req),
                Err(err) => {
                    error!("Failed to parse bad request. Error: {err}");
                    Err(())
                }
            },
            Err(err) => {
                error!("Failed to read protocol request from tcp stream. Error: {err}");
                Err(())
            }
        }
    }

    fn handle_read(&mut self, req: ReadRequest) {
        info!("Read request received");
        let entity = match self.entities.get(&req.entity_name) {
            Some(entity) => entity,
            None => {
                error!(
                    "Read request failed because entity {} in service {} does not exist",
                    req.entity_name, req.service_name
                );
                return;
            }
        };
        let res = match entity.read() {
            Ok(res) => res,
            Err(err) => Response::Error(err.into()),
        };
        match self.stream.write_all(&res.to_bytes()) {
            Ok(()) => info!("Sent response to tcp stream; read request successful"),
            Err(err) => error!("Failed to write read request to tcp server. Error: {err}"),
        };
    }

    fn handle_write(&mut self, req: WriteRequest) {
        info!("Write req received");
        let entity = match self.entities.get_mut(&req.entity_name) {
            Some(entity) => entity,
            None => {
                error!(
                    "Write request failed because entity {} in service {} does not exist",
                    req.entity_name, req.service_name
                );
                return;
            }
        };
        let res = match entity.write(&req.data) {
            Ok(res) => res,
            Err(err) => Response::Error(err.into()),
        };
        match self.stream.write_all(&res.to_bytes()) {
            Ok(()) => info!("Sent response to tcp stream; write request successful"),
            Err(err) => {
                error!("Failed to send write request to tcp server. Error: {err}")
            }
        };
    }
}
