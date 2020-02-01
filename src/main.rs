mod err;

use std::env;
use std::fs;
use std::path::Path;
use std::io::{Read, Write};
use err::handle;
use err::err;
use bluetooth_serial_port::{BtProtocol, BtSocket, BtDevice, BtAddr};

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
    match socket.connect(device.addr) {
        Ok(_) => {},
        Err(why) => {
            println!("Failed to connect to {}: {}", device.addr.to_string(), why);
            std::process::exit(1);
        }
    };

    // Read the file to bytes
    let bytes = handle(fs::read(Path::new(&args[3])), format!("Error reading file {}", &args[3]).as_str());
    
    // Data buf for IO
    let mut buffer = [0; 10];

    // Read and write data over the connection
    let num_bytes_written = match socket.write_all(&bytes[0..]) {
        Ok(e) => e,
        Err(why) => {
            eprintln!("Error writing bytes over socket: {}", why);
            std::process::exit(1);
        }
    };
    socket.write(b"EOF").expect("Failed to write to socket");

    // Read data from connection 
    let num_bytes_read = match socket.read(&mut buffer[..]) {
        Ok(e) => e,
        // If this fails, don't panic, just return 0 and print an err
        Err(why) => {
            eprintln!("Failed to read from socket ({}); continuing", why); 0
        }
    };

    println!("Read {} bytes, wrote {:?} bytes", num_bytes_read, num_bytes_written);
}
