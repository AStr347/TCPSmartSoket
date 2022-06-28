use std::{
    io::{self, Read, Write},
    net::Shutdown,
};

use TCPClient::TCPClient;

fn read() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read from stdin");
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

pub struct CLIClient {}

impl TCPClient for CLIClient {
    fn connection_handler(&mut self, stream: &mut std::net::TcpStream) {
        let s = read();
        let dst_id = u64::from_str_radix(&s, 16).unwrap();
        stream
            .write(&dst_id.to_be_bytes())
            .expect("can't write dst_id in client stream");
        let mut buff: [u8; 1] = [0; 1];
        let readres = stream.read(&mut buff);
        if let Err(e) = readres {
            println!("{}", e);
            stream
                .shutdown(Shutdown::Both)
                .expect("can't shutdown client stream");
            return;
        }
        if 0 != buff[0] {
            stream
                .shutdown(Shutdown::Both)
                .expect("can't shutdown client stream");
            return;
        }
        println!(" SmartSoket connected ");
        println!("0 - Off ");
        println!("1 - On");
        println!("2 - Get status");
        println!("_ - close connection");
        let mut buff: [u8; 128] = [0; 128];
        loop {
            let s = read();
            let comres = s.parse::<u8>();
            if let Err(e) = comres {
                println!("{}", e);
                return;
            }
            let com = comres.unwrap();
            if (3 < com) {
                stream
                    .shutdown(Shutdown::Both)
                    .expect("can't shutdown client stream");
                return;
            } else {
                let payload = [com];
                stream.write(&payload).expect("can't write client stream");

                buff.fill(0);
                let readres = stream.read(&mut buff);
                if let Err(e) = readres {
                    println!("{}", e);
                    stream
                        .shutdown(Shutdown::Both)
                        .expect("can't shutdown client stream");
                    return;
                }
                let s = String::from_utf8_lossy(&buff);
                println!("{}", s);
            }
        }
    }
}
