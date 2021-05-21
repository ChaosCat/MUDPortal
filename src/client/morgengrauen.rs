use telnet::{Telnet, TelnetEvent, TelnetOption, NegotiationAction};
use crate::network::telnet::comm::{echo_action_agreement, handle_linemode_sb};
use std::io::{stdout, stdin, Write, BufRead};
use crate::network::telnet::consts::TELNET_OPTION_TLS_CODE;

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
    loop {
        curr_event = connection.read().expect("Failed reading from server");
        match curr_event {
            TelnetEvent::Data(data) => {
                println!("Received data");
                lout.write_all(data.as_ref())?;
            },
            TelnetEvent::UnknownIAC(iac_id) => {
                eprintln!("Error: unrecognized IAC: {}", iac_id);
            },
            TelnetEvent::Negotiation(action, option) => {
                println!("Received negotiaition: {:?} {:?}", action, option);
                match option {
                    TelnetOption::TransmitBinary => {
                        println!("Agreeing to binary transmission");
                        echo_action_agreement(&mut connection, action, option);
                    },
                    TelnetOption::EOR => {
                        println!("Agreeing to End-Of-Record transmission");
                        echo_action_agreement(&mut connection, action, option);
                    },
                    TelnetOption::NAWS => {
                        println!("Agreeing to window-size subnegotiation (without initiating one)");
                        echo_action_agreement(&mut connection, action, option);
                    },
                    TelnetOption::TTYPE => {
                        println!("Rejecting Terminal Type option");
                        //echo_action_agreement(&mut connection, action, option);
                        connection.negotiate(NegotiationAction::Wont, TelnetOption::TTYPE);
                    }
                    TelnetOption::Linemode => {
                        println!("Received Linemode subnegotiation, rejecting");
                        // echo_action_agreement(&mut connection, action, option);
                        connection.negotiate(NegotiationAction::Wont, TelnetOption::Linemode)
                    },
                    TelnetOption::UnknownOption(unk_value) => {
                        if unk_value == TELNET_OPTION_TLS_CODE {
                            println!("TLS requested, rejecting");
                            connection.negotiate(NegotiationAction::Wont,
                                                 TelnetOption::UnknownOption(
                                                     TELNET_OPTION_TLS_CODE));
                        } else {
                            eprintln!("Unknown option received: {}, rejecting", option.to_byte());
                            connection.negotiate(NegotiationAction::Wont, option)
                        }
                    },
                    _ => {
                        eprintln!("Option: {} unsupported, rejecting", option.to_byte());
                        connection.negotiate(NegotiationAction::Wont, option)
                    }
                };
            },
            TelnetEvent::Subnegotiation(option, subnegotiation_data) => {
                println!("Received subnegotiation for option: {:?}", option);
                print!("\tData: ");
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
                eprintln!("Connection timed out~!");
            },
            TelnetEvent::NoData => {
                eprintln!("Telnet NoData received")
            },
            TelnetEvent::Error(err) => {
                eprintln!("Error: {}", err)
            }
        };
        lout.flush();
        lin.read_line(&mut input).expect("Failed reading user input");
        print!("{}", input);
        if input.contains("##") {
            break;
        }
        if input.contains("!!") {
            println!("Negotiating binary transmission");
            connection.write(input.replace("!!", "").as_bytes());
        }
        input.clear();
    }

    Ok(())
}