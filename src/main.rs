mod err;

use std::env;
use std::fs;
use std::path::Path;
use err::handle;
use err::err;
use bluetooth_serial_port::{BtProtocol, BtSocket, BtDevice, BtAddr};
use mio::{Poll, PollOpt, Ready, Token};
use std::{
    io::{Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        err("Usage: ssssctl <name> <address> <file>", 1);
    }

    let name = &args[1];
    let device = BtDevice::new(name.to_string(), handle(BtAddr::from_str(&args[2]), "Failed to parse MAC address"));

    println!("Connecting to \"{}\" ({})", device.name, device.addr.to_string());

    // create and connect the RFCOMM socket
    let mut socket = handle(BtSocket::new(BtProtocol::RFCOMM), "Failed to create new bluetooth socket");

    handle(socket.connect(device.addr), format!("Failed to connect to {}", device.addr.to_string()).as_str());

    let bytes = handle(fs::read(Path::new(&args[3])), "Error reading file");
    
    // Data buf for IO
    let mut buffer = [0; 10];

    // Read and write data over the connection
    let num_bytes_written = match socket.write(&bytes[..]) {
        Ok(e) => e,
        Err(why) => {
            eprintln!("Error writing bytes over socket: {}", why);
            std::process::exit(1);
        }
    };

    let num_bytes_read = match socket.read(&mut buffer[..]) {
        Ok(e) => e,
        // If this fails, don't panic, just return 0 and print an err
        Err(why) => {
            eprintln!("Failed to read from socket; continuing {}", why); 0
        }
    };

    println!("Read {} bytes, wrote {} bytes", num_bytes_read, num_bytes_written);

    // Asnyc IO for BtSocket 
    let poll = Poll::new().expect("Failed to create poll");
    match poll.register(
        &socket,
        Token(0),
        Ready::readable() | Ready::writable(),
        PollOpt::edge() | PollOpt::oneshot(),
    ) {
        Ok(_) => {},
        Err(why) => {
            eprintln!("Failed to register: {}", why);
            std::process::exit(1);
        }
    };
}
