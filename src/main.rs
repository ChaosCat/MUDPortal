pub mod client;
pub mod network;

use telnet::{Telnet, TelnetEvent, NegotiationAction, TelnetOption};
use std::io::{stdin, stdout, Write, BufRead};
use std::convert::TryFrom;
use std::fmt::{Error, Display, Formatter};
use crate::client::morgengrauen::client_loop;
use crate::client::consts::MORGENGRUEN_CONNECTION_INFO;

// TODO: A specialized error encompassing possibilities


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut connection = Telnet::connect(MORGENGRUEN_CONNECTION_INFO,
                                         1028).expect("Server connection failed");

    client_loop(connection);

    Ok(())
}
