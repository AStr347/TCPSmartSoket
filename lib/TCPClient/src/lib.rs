use std::io::Read;
use std::net::TcpStream;

pub trait TCPClient {
    fn connection_handler(&mut self, stream: &mut TcpStream);

    fn client_runner(&mut self, addr: &str) {
        let connection = TcpStream::connect(addr);
        match connection {
            Ok(mut stream) => {
                println!("Successfully connected to server");
                self.connection_handler(&mut stream);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        println!("Terminated.");
    }

    fn read_buff(stream: &mut TcpStream) -> Option<(usize, [u8; 128])> {
        let mut data: [u8; 128] = [0; 128];
        let readres = stream.read(&mut data);
        match readres {
            Ok(readed) => Some((readed, data)),
            Err(_) => None,
        }
    }
}
