use std::{time::Duration, sync::{Mutex, Arc}, io::{self, Error}};

use tokio::{
    net::{
        TcpListener, 
        TcpStream
    },
    io::{AsyncWriteExt, AsyncReadExt},
    time::sleep
};

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

    
    pub async fn serv_runner(self, addr: &'static str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        let me = Arc::new(Mutex::new(self));
        loop {
            let me = me.clone();
            match listener.accept().await{
                Ok((stream, addr)) => {
                    println!("Connection established! from {}", addr);
                    tokio::spawn(
                        async move {
                            Self::stream_handler(me.clone(), stream).await;
                        }
                    ).await.unwrap();
                },
                Err(e) => println!("{}", e),
            }
        }
    }

    async fn stream_handler(soket : Arc<Mutex<Self>>, mut stream: TcpStream) {
        println!("wait id");
        loop {
            let soket = soket.clone();
            let mut buff: [u8; 128] = [0; 128];
            let readres = stream.try_read(&mut buff);
            match readres {
                Ok(siz) => {
                    println!("Request: readed: {} payload: {:?}", siz, &buff[0..16]);
                    if 8 > siz {
                        stream
                            .shutdown()
                            .await
                            .expect("can't shutdown server stream");
                        return;
                    }
                    let dst_b: [u8; 8] = buff[0..8].try_into().unwrap();
                    let dst_id = u64::from_be_bytes(dst_b);
                    let id : u64 = 
                    {
                        let s = soket.lock().unwrap();
                        s.id
                    };
                    if id != dst_id {
                        stream
                            .shutdown()
                            .await
                            .expect("can't shutdown server stream");
                        return;
                    } else {
                        break;
                    }
                },
                Err(e) => println!("90: {}", e),
            }
            sleep(Duration::from_secs(3)).await;
        }
        Self::client_handler(soket, stream).await;
        println!("connection closed");
    }

    async fn err_shutdown(mut stream: TcpStream, e:Error){
        println!("120: {}", e);
        stream
            .shutdown()
            .await
            .expect("can't shutdown server stream");
        return;
    }

    async fn client_handler(soket : Arc<Mutex<Self>>, mut stream: TcpStream) {
        stream
            .write(&[0]).await
            .expect("can't write answer, server stream");
        loop {
            sleep(Duration::from_secs(3)).await;
            let err = stream.take_error();
            match err {
                Ok(None) => {},
                Ok(Some(oe)) => { Self::err_shutdown(stream, oe).await; return; },
                Err(e) => { Self::err_shutdown(stream, e).await; return; }
            }
            
            let mut buf : &mut [u8] = &mut [0; 10];
            let readres = stream.try_read(&mut buf);
            match readres {
                Ok(siz) => {
                    if 0 < siz {
                        let com = SmartSoketCommands::new(buf[0]);
                        let answer = {
                            let mut s = soket.lock().unwrap();
                            s.exec(com)
                        };
                        println!("{}", answer);
                        stream
                            .write(answer.as_bytes()).await
                            .expect("can't write answer, server stream");
                    } else {
                        println!("readed 0");
                        println!("{:?}", buf);
                        // stream
                        //     .shutdown()
                        //     .await
                        //     .expect("can't shutdown server stream");
                        // return;
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => { println!("{}", e); continue; }
                Err(e) => { Self::err_shutdown(stream, e).await; return; }
            }
        }
    }
}
