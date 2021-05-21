use telnet::{Telnet, NegotiationAction, TelnetOption};
use std::fmt::{Display, Formatter};

pub enum LinemodeMode {
    Edit,
    Unrecognized(u8)
}

#[repr(u8)]
pub enum Linemode {
    Mode,
    ForwardMask,
    Slc,
    Unrecognized(u8),
}

impl std::convert::From<u8> for Linemode {
    fn from(value: u8) -> Self {
        match value {
            1 => Linemode::Mode,
            2 => Linemode::ForwardMask,
            3 => Linemode::Slc,
            _ => Linemode::Unrecognized(value),
        }
    }
}
impl std::convert::Into<u8> for Linemode {
    fn into(self) -> u8 {
        match self {
            Linemode::Mode => 1,
            Linemode::ForwardMask => 2,
            Linemode::Slc => 3,
            Linemode::Unrecognized(value) => value
        }
    }
}

pub fn echo_action_agreement(connection: &mut Telnet, action: NegotiationAction, option: TelnetOption) {
    match action {
        NegotiationAction::Do => connection.negotiate(NegotiationAction::Will, option),
        NegotiationAction::Dont => connection.negotiate(NegotiationAction::Wont, option),
        NegotiationAction::Will => connection.negotiate(NegotiationAction::Do, option),
        NegotiationAction::Wont => connection.negotiate(NegotiationAction::Dont, option),
    }
}

#[derive(Debug)]
pub enum LinemodeSBError {
    PrimaryCommandConversion,
}

impl Display for LinemodeSBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LineModeSBError")
    }
}

impl std::convert::From<&Linemode> for String {
    fn from(linemode: &Linemode) -> Self {
        match linemode {
            Linemode::Mode => String::from("Mode"),
            Linemode::ForwardMask => String::from("ForwardMask"),
            Linemode::Slc => String::from("Slc"),
            Linemode::Unrecognized(val) => val.to_string()
        }.parse().unwrap()
    }
}

impl std::convert::From<&LinemodeMode> for String {
    fn from(lmm: &LinemodeMode) -> Self {
        match lmm {
            LinemodeMode::Edit => String::from("Edit"),
            LinemodeMode::Unrecognized(val) => val.to_string()
        }.parse().unwrap()
    }
}

impl std::convert::From<u8> for LinemodeMode {
    fn from(val: u8) -> Self {
        match val {
            1 => LinemodeMode::Edit,
            _ => LinemodeMode::Unrecognized(val),
        }
    }
}

impl Display for Linemode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", String::from(self)) }
}

impl Display for LinemodeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", String::from(self)) }
}

impl std::error::Error for LinemodeSBError {}

// TODO: Properly handle Subnegotiations, currently just ignoring them all
pub fn handle_linemode_sb(connection: &mut Telnet, data: Box<[u8]>) -> Result<(), LinemodeSBError> {
    let linemode_primary_option: Linemode = Linemode::from(data[0]);
    match linemode_primary_option {
        Linemode::Mode => {
            println!("Received Linemode MODE command");

            let linemode_mode: LinemodeMode = LinemodeMode::from(data[1]);
            println!("Linemode mode: {}", linemode_mode);
            match linemode_mode {
                LinemodeMode::Edit => {
                    connection.subnegotiate(TelnetOption::Linemode, &[1, 1])
                },
                LinemodeMode::Unrecognized(_val) => {
                    // Ignore
                }
            }
        }
        _ => {
            eprintln!("Unrecognized Linemode primary option: {}", linemode_primary_option)
        }
    }
    Ok(())
}