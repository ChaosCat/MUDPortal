#[macro_use] extern crate log;
#[macro_use] extern crate simple_logger;

pub mod client;
pub mod network;

use telnet::{Telnet, TelnetEvent, NegotiationAction, TelnetOption};
use std::io::{stdin, stdout, Write, BufRead};
use std::convert::TryFrom;
use std::fmt::{Error, Display, Formatter};
use crate::client::morgengrauen::client_loop;
use crate::client::consts::MORGENGRUEN_CONNECTION_INFO;
use simple_logger::SimpleLogger;
use log::SetLoggerError;

// TODO: A specialized error encompassing possibilities


fn main() -> Result<(), Box<dyn std::error::Error>>{

    match SimpleLogger::new().init() {
        Ok(_) => info!("Logger initialized"),
        Err(err) => {
            eprintln!("Logger initialization failed with error: {}", err);
            return Ok(())
        }
    }

    let mut connection = match Telnet::connect(MORGENGRUEN_CONNECTION_INFO,
                                         1028) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Connection establishment failed!");
            return Ok(())
        }
    };
    
    client_loop(connection);

    Ok(())
}
