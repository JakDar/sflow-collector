extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sflow;
#[macro_use]
extern crate clap;

use model::mega_packet::PacketJson;
use sflow::Decodeable;
use sflow::flow_records::SampledHeader;
use std::io::Cursor;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use clap::{Arg, App, SubCommand};

mod model {
    pub mod l3;
    pub mod l4;
    pub mod l7;
    pub mod mega_packet;
}

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

fn start_forwarding_server(receiver: Receiver<String>, server_socket: &str) {
    let server_listener = TcpListener::bind(server_socket).unwrap();
    println!("{:?}", server_listener.local_addr().unwrap());
    println!("Started tcp server on {}", server_socket);

    for stream in server_listener.incoming() {
        println!("Client connected!");

        let mut client: TcpStream = stream.unwrap();
        loop {
            let message = receiver.recv().unwrap() + "\n";
            if let Err(e) = client.write(message.as_bytes()).and_then(|_| client.flush()) {
                println!("Error while writing to a socket: {}\n", e);
                break;
            }
        }
    }
}

fn read_incoming_packets(channel: Sender<String>, addr: &str) {
    let incoming_socket = UdpSocket::bind(addr).unwrap();
    let mut buffer = [0u8; 1500];

    loop {
        let packet: &mut [u8] = &mut buffer;
        let (size, _) = match incoming_socket.recv_from(packet) {
            Err(e) => {
                println!("Failed to read from socket: {:?}", e);
                continue;
            }
            Ok(some) => some
        };

        if size > 1500 {
            println!("Packet too large.");
            continue;
        }

        let mut packet_cursor = Cursor::new(packet);
        let dgram: sflow::Datagram = match sflow::Datagram::read_and_decode(&mut packet_cursor) {
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
                    .map(|header| PacketJson::from_sampled_header(header.unwrap(), flow.sampling_rate))
                    .map(|x| serde_json::to_string(&x))
                    .for_each(|s| {
                        let json: String = s.unwrap();
                        println!("{}", &json.clone());
                        channel.send(json); // todo - should return Result
                    }),
                _ => ()
            }
        }
    }
}


fn main() {
    let matches = App::new("Sflow Collector")
        .version("0.1")
        //todo - wording
        .about("Receives sflow UDP packets, parses sampledHeaderBytes to human-readable data, and creates server that is going to pass that data in json.")
        .arg(Arg::with_name("Sflow Port")
            .required(true)
            .short("p")
            .long("sflow-port")
            .help("Port from which to read sflow packets.")
            .takes_value(true)
        )
        .arg(Arg::with_name("Server port")
            .short("l")
            .long("server-listen-port")
            .help("Port for tcp server serving parsed sflows as json")
            .default_value("9999")
            .takes_value(true)
        )
        .get_matches();

    //Todo - handle gracefully
    let sflow_port: u32 = value_t!(matches,"Sflow Port",u32).unwrap();
    let server_port: u32 = value_t!(matches,"Server port",u32).unwrap();

    //todo - add commandline args
    let incoming_udp_socket = format!("0.0.0.0:{}", sflow_port);
    let outgoing_tcp_socket = format!("0.0.0.0:{}", server_port);

    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    std::thread::spawn(move || read_incoming_packets(sender, incoming_udp_socket.as_str()));
    println!("Now sending");
    start_forwarding_server(receiver, outgoing_tcp_socket.as_str());
}
