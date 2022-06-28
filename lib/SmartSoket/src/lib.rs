use std::{
    io::{Read, Write},
    net::Shutdown,
    time::Duration,
};

use TCPServer::TCPServer;

#[derive(Debug)]
enum SmartSoketCommands {
    Off,
    On,
    Get,
}

impl SmartSoketCommands {
    fn new(com: u8) -> Self {
        match com {
            0 => Self::Off,
            1 => Self::On,
            _ => Self::Get,
        }
    }
}

pub struct SmartSoket {
    id: u64,
    state: bool,
}

impl SmartSoket {
    pub fn new(id: u64) -> Self {
        Self { id, state: false }
    }
    fn exec(&mut self, com: SmartSoketCommands) -> String {
        match com {
            SmartSoketCommands::Off => {
                self.state = false;
                let s = format!("off - id:{} state:{}", self.id, self.state);
                s
            }
            SmartSoketCommands::On => {
                self.state = true;
                let s = format!("on - id:{} state:{}", self.id, self.state);
                s
            }
            SmartSoketCommands::Get => {
                let s = format!("get - id:{} state:{}", self.id, self.state);
                s
            }
        }
    }
}

impl TCPServer for SmartSoket {
    fn client_handler(&mut self, stream: &mut std::net::TcpStream, buffer: (usize, &[u8])) {
        let (siz, buff) = buffer;
        if (8 > siz) {
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }
        let dst_b: [u8; 8] = buff[0..8].try_into().unwrap();
        let dst_id = u64::from_be_bytes(dst_b);
        if (self.id != dst_id) {
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }
        stream
            .write(&[0])
            .expect("can't write answer, server stream");
        let mut buf: [u8; 4] = [0; 4];
        stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .unwrap();

        loop {
            let err = stream.take_error();
            match err {
                Ok(oe) => match oe {
                    Some(e) => {
                        println!("{}", e);
                        stream
                            .shutdown(Shutdown::Both)
                            .expect("can't shutdown server stream");
                        return;
                    }
                    None => {}
                },
                Err(e) => {
                    println!("{}", e);
                    stream
                        .shutdown(Shutdown::Both)
                        .expect("can't shutdown server stream");
                    return;
                }
            }

            let readres = stream.read(&mut buf[..]);
            match readres {
                Ok(siz) => {
                    if (0 < siz) {
                        let com = SmartSoketCommands::new(buf[0]);
                        let answer = self.exec(com);
                        println!("{}", answer);
                        stream
                            .write(answer.as_bytes())
                            .expect("can't write answer, server stream");
                    } else {
                        stream
                            .shutdown(Shutdown::Both)
                            .expect("can't shutdown server stream");
                        return;
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    stream
                        .shutdown(Shutdown::Both)
                        .expect("can't shutdown server stream");
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod sokettest {
    use super::*;
    #[test]
    fn runner() {
        let mut soket = SmartSoket::new(0xDEADBEAF);
        soket.serv_runner("127.0.0.1:55331")
    }
}
