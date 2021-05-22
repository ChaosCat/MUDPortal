#[macro_use] extern crate log;
extern crate simple_logging;

pub mod client;
pub mod network;

use telnet::Telnet;
use crate::client::morgengrauen::client_loop;
use crate::client::consts::MORGENGRUEN_CONNECTION_INFO;
use log::LevelFilter;

// TODO: A specialized error encompassing possibilities


fn main() -> Result<(), Box<dyn std::error::Error>>{


    match  simple_logging::log_to_file("logs/trace.log", LevelFilter::Trace) {
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
            eprintln!("Connection establishment failed with error: {}", err);
            return Ok(())
        }
    };
    
    client_loop(connection);

    Ok(())
}
