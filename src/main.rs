#![allow(unused_variables)]
#![allow(unused_imports)]

use std::net::{Shutdown, TcpStream};
use std::ops::Range;

fn scan(host: String, port: u32) {
    let host_port = format!("{}:{}", host, port);
    let stream = TcpStream::connect(host_port);
    match stream {
        Ok(stream) => {
            println!("{:?}", port);
            stream.shutdown(Shutdown::Both).expect("");
        }
        Err(stream) => {}
    }
}

fn main() {
    let from = 1;
    let to = 65535;
    let host = String::from("127.0.0.1");
    let r = Range {
        start: from,
        end: to + 1,
    };
    println!("Open ports");
    println!("----------");
    for i in r {
        let _host = host.clone();
        thread::spawn(move || scan(_host, i as u32));
    }
}
