extern crate sflow;
use std::net::UdpSocket;
use std::io::BufReader;
use std::io::Cursor;
use sflow::Decodeable;


fn main() {
    let mut stream = UdpSocket::bind("0.0.0.0:1337").unwrap();
    
    let mut buffer = [0u8; 1500];

    loop {
        let mut packet : &mut [u8] = &mut buffer;
        let (size, remote) = match stream.recv_from(packet) {
            Err(e) => {
                println!("Failed to read from socket: {:?}", e);
                continue;
            }
            Ok(some) => some
        };

        println!("Packet size: {}", size);

        if size > 1500 {
            println!("Packet too large.");
            continue;
        }

        let mut cur = Cursor::new(packet);
        let dgram = match sflow::Datagram::read_and_decode(&mut cur) {
            Err(e) => {
                println!("failed to decode sample: {:?}", e);
                continue
            }
            Ok(some) => some
        };
        for sample in &dgram.sample_record {
            println!("Sample: {:?}", sample);
        }
    }
}
