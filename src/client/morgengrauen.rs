use telnet::{Telnet, TelnetEvent, TelnetOption, NegotiationAction};
use crate::network::telnet::comm::{echo_action_agreement, handle_linemode_sb};
use std::io::{stdout, stdin, Write, BufRead, Error};
use crate::network::telnet::consts::TELNET_OPTION_TLS_CODE;
use std::any::Any;

// TODO: Implement to be context sensitive
struct MorgenGrauenClient {

}

pub fn client_loop(mut connection: Telnet) -> Result<(), Box<dyn std::error::Error>>{
    let mut input: String = String::new();
    let mut curr_event: TelnetEvent;
    let stdout = stdout();
    let stdin = stdin();
    let mut lout = stdout.lock();
    let mut lin = stdin.lock();
    let mut should_request_input= false;
    loop {
        curr_event = connection.read().expect("Failed reading from server");
        match curr_event {
            TelnetEvent::Data(data) => {
                trace!("Received data");
                lout.write_all(data.as_ref())?;
                should_request_input = true;
            },
            TelnetEvent::UnknownIAC(iac_id) => {
                error!("Error: unrecognized IAC: {}", iac_id);
            },
            TelnetEvent::Negotiation(action, option) => {
                info!("Received negotiaition: {:?} {:?}", action, option);
                match option {
                    TelnetOption::TransmitBinary => {
                        info!("Agreeing to binary transmission");
                        echo_action_agreement(&mut connection, action, option);
                    },
                    TelnetOption::EOR => {
                        info!("Agreeing to End-Of-Record transmission");
                        echo_action_agreement(&mut connection, action, option);
                    },
                    TelnetOption::NAWS => {
                        info!("Agreeing to window-size subnegotiation (without initiating one)");
                        echo_action_agreement(&mut connection, action, option);
                    },
                    TelnetOption::TTYPE => {
                        info!("Rejecting Terminal Type option");
                        //echo_action_agreement(&mut connection, action, option);
                        connection.negotiate(NegotiationAction::Wont, TelnetOption::TTYPE);
                    }
                    TelnetOption::Linemode => {
                        info!("Received Linemode subnegotiation, rejecting");
                        // echo_action_agreement(&mut connection, action, option);
                        connection.negotiate(NegotiationAction::Wont, TelnetOption::Linemode)
                    },
                    TelnetOption::UnknownOption(unk_value) => {
                        if unk_value == TELNET_OPTION_TLS_CODE {
                            warn!("TLS requested, rejecting");
                            connection.negotiate(NegotiationAction::Wont,
                                                 TelnetOption::UnknownOption(
                                                     TELNET_OPTION_TLS_CODE));
                        } else {
                            error!("Unknown option received: {}, rejecting", option.to_byte());
                            connection.negotiate(NegotiationAction::Wont, option)
                        }
                    },
                    _ => {
                        error!("Option: {} unsupported, rejecting", option.to_byte());
                        connection.negotiate(NegotiationAction::Wont, option)
                    }
                };
            },
            TelnetEvent::Subnegotiation(option, subnegotiation_data) => {
                trace!("Received subnegotiation for option: {:?}", option);
                trace!("\tData: ");
                for c in subnegotiation_data.as_ref() {
                    print!("{:02x} ", *c);
                }
                println!();
                match option {
                    TelnetOption::Linemode => handle_linemode_sb(&mut connection, subnegotiation_data)?,
                    _ => {
                        eprintln!("No handler implemented yet for this subnegotiation");
                    }
                }
            },
            TelnetEvent::TimedOut  => {
                error!("Connection timed out~!");
            },
            TelnetEvent::NoData => {
                error!("Telnet NoData received")
            },
            TelnetEvent::Error(err) => {
                error!("Error: {}", err)
            }
        };
        lout.flush();

        // Request input only on Data Telnet Event
        if should_request_input {
            should_request_input = false;
            match lin.read_line(&mut input) {
                Ok(bytes_read) => {
                    trace!("User input ({} bytes): {}", bytes_read, input);
                    connection.write(input.as_bytes());
                }
                Err(err) => {
                    should_request_input = true;
                    error!("Failed reading stdin: {}", err)
                }
            }
        }
        input.clear();
    }

    Ok(())
}