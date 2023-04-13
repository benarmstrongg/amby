use std::{io::Write, net::TcpStream, process};

use log::{error, info};

use crate::{App, ClientType, Protocol, ReadAll, Response};

pub trait RegisterClient {
    fn get_client_type() -> ClientType;

    fn register_client(mut stream: &TcpStream, data: &[u8]) {
        let client_type = match Self::get_client_type() {
            ClientType::App => "app",
            ClientType::Protocol => "protocol",
        };
        match stream.write_all(data) {
            Ok(()) => {
                info!("Wrote {client_type} name to tcp stream. Waiting for response",);
            }
            Err(err) => {
                error!("Error writing {client_type} name to tcp stream. Error: {err}");
                process::exit(1);
            }
        };
        match stream.read_all() {
            Ok(res) => {
                match Response::from(res) {
                    Response::Success(_data) => {
                        info!("Received success response from tcp stream; {client_type} registered")
                    }
                    Response::Error(_err) => {
                        error!("Received error response from tcp stream; {client_type} was not registered");
                        process::exit(1);
                    }
                    Response::Empty => {
                        error!("Received empty response from tcp stream; {client_type} was not registered");
                        process::exit(1);
                    }
                }
            }
            Err(err) => {
                error!("Failed to read response from tcp stream; {client_type} could not be registered. Error: {err}");
                process::exit(1);
            }
        }
    }
}

impl RegisterClient for App {
    fn get_client_type() -> ClientType {
        ClientType::App
    }
}

impl RegisterClient for Protocol {
    fn get_client_type() -> ClientType {
        ClientType::Protocol
    }
}
