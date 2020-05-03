pub mod scanner {

    use std::net::{Shutdown, TcpStream};

    pub fn scan(host: String, port: u32) {
        let host_port = format!("{}:{}", host, port);
        let stream = TcpStream::connect(host_port);
        match stream {
            Ok(stream) => {
                println!("{:?}", port);
                stream.shutdown(Shutdown::Both).expect("");
            }
            Err(_stream) => {}
        }
    }
}
