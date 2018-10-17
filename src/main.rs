extern crate sflow;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod model {
    pub mod l3;
    pub mod l4;
    pub mod l7;
    pub mod mega_packet;
}

use model::mega_packet::PacketJson;
use std::net::UdpSocket;
use std::io::BufReader;
use std::io::Cursor;
use sflow::Decodeable;
use serde_json::Error;
use sflow::flow_records::SampledHeader;


fn print_flow_record(record: &sflow::FlowRecord) {
    use sflow::FlowRecord::*;
    match record {
        SampledHeader(sample) => println!("Sample: {:?}", sample),
        SampledIpv4(sample) => println!("Sample: {:?}", sample),
        _ => ()
    }
}

fn get_sampled_header(record: &sflow::FlowRecord) -> Option<&SampledHeader> {
    use sflow::FlowRecord::*;
    match record {
        SampledHeader(sample) => Some(sample),
        _ => None
    }
}


fn main() {
//    let json = serde_json::to_string(&x).unwrap();



    let mut stream = UdpSocket::bind("0.0.0.0:6343").unwrap();

    let mut buffer = [0u8; 1500];

    loop {
        let mut packet: &mut [u8] = &mut buffer;
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
        let dgram: sflow::Datagram = match sflow::Datagram::read_and_decode(&mut cur) {
            Err(e) => {
                println!("failed to decode sample: {:?}", e);
                continue;
            }
            Ok(some) => some
        };
        for sample in &dgram.sample_record {
            match sample {
                sflow::SampleRecord::FlowSample(flow) => flow.flow_records.iter()
                    .map(|record| get_sampled_header(record))
                    .filter(|header| header.is_some())
                    .map(|header| PacketJson::from_sampled_header(header.unwrap()))
                    .map(|x| serde_json::to_string(&x))
                    .for_each(|s| println!("{}", s.unwrap())),
                _ => ()
            }
        }
    }
}
