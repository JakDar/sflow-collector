use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;


fn read_stdin(chan: Sender<String>) {
    let input = io::stdin();

    thread::spawn(move || {
        for line in input.lock().lines() {
            let _ = chan.send(line.unwrap());
        }
    });
}


fn start_listening(receiver: Receiver<String>) {
//    fn handle_client(mut stream: TcpStream) {
//        thread::spawn(move || {
//            loop {
//                stream.write("Hello World!\n".as_bytes());
//            }
//        });
//    }

    let listener = TcpListener::bind("0.0.0.0:9999").unwrap();

    for stream in listener.incoming() {
        println!("Client connected!");

        let mut client = stream.unwrap();
        loop {
            let message = receiver.recv().unwrap() + "\n";
            if let Err(e) = client.write(message.as_bytes()).and_then(|_| client.flush()) {
                println!("Error while writing to a socket: {}\n", e);
                break;
            }
        }
    }
}

fn main() {
    println!("Socket forwarding successfully started!");

    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
    read_stdin(sender);
    start_listening(receiver);
}