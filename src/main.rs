use std::env;
use std::fs;
use std::path::Path;
use bluetooth_serial_port::{BtProtocol, BtSocket, BtDevice, BtAddr};
use mio::{Poll, PollOpt, Ready, Token};
use std::{
    io::{Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: btd <name> <address> <file>");
        std::process::exit(1);
    }

    let name = &args[1];
    let device = BtDevice::new(name.to_string(), BtAddr::from_str(&args[2]).expect("Failed to convert arg to MAC addr"));

    println!("Connecting to \"{}\" ({})", device.name, device.addr.to_string());

    // create and connect the RFCOMM socket
    let mut socket = BtSocket::new(BtProtocol::RFCOMM).expect("Failed to make bluetooth socket");
    match socket.connect(device.addr) {
        Ok(_) => {},
        Err(why) => {
            println!("Got error {}", why);
            std::process::exit(1);
        }
    };

    let bytes = match fs::read(Path::new(&args[3])) {
        Ok(d) => d,
        Err(why) => {
            println!("Error reading file...{}", why);
            std::process::exit(1);
        }
    };
    
    // Data buf for IO
    let mut buffer = [0; 10];

    // Read and write data over the connection
    let num_bytes_written = socket.write(&bytes[..]).unwrap();
    let num_bytes_read = match socket.read(&mut buffer[..]) {
        Ok(e) => e,
        Err(why) => {
            println!("Got error {}", why);
            0
        }
    };

    println!("Read {} bytes, wrote {} bytes", num_bytes_read, num_bytes_written);

    // BtSocket also implements  for async IO
    let poll = Poll::new().unwrap();
    poll.register(
        &socket,
        Token(0),
        Ready::readable() | Ready::writable(),
        PollOpt::edge() | PollOpt::oneshot(),
    ).unwrap();
}
