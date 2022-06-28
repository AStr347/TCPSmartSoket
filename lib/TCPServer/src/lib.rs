use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub trait TCPServer {
    fn serv_runner(&mut self, addr: &'static str) {
        let listener = TcpListener::bind(addr).unwrap();

        for incom in listener.incoming() {
            match incom {
                Ok(mut stream) => {
                    println!("Connection established!");
                    self.stream_handler(&mut stream);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }

    fn stream_handler(&mut self, stream: &mut TcpStream) {
        let mut buff: [u8; 128] = [0; 128];

        let readres = stream.read(&mut buff);

        if let Ok(readed) = readres {
            println!("Request: readed: {} payload: {:?}", readed, &buff[0..16]);

            self.client_handler(stream, (readed, &buff[..]));

            println!("connection closed");
        }
    }

    fn client_handler(&mut self, stream: &mut TcpStream, buffer: (usize, &[u8]));
}
