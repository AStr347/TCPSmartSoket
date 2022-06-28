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

// fn main() {
//     match TcpStream::connect("localhost:3333") {
//         Ok(mut stream) => {
//             println!("Successfully connected to server in port 3333");

//             let msg = b"Hello!";

//             stream.write(msg).unwrap();
//             println!("Sent Hello, awaiting reply...");

//             let mut data = [0 as u8; 6]; // using 6 byte buffer
//             match stream.read_exact(&mut data) {
//                 Ok(_) => {
//                     if &data == msg {
//                         println!("Reply is ok!");
//                     } else {
//                         let text = from_utf8(&data).unwrap();
//                         println!("Unexpected reply: {}", text);
//                     }
//                 },
//                 Err(e) => {
//                     println!("Failed to receive data: {}", e);
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Failed to connect: {}", e);
//         }
//     }
//     println!("Terminated.");
// }
