use SmartSoket::SmartSoket;
use TCPServer::TCPServer;

fn main() {
    let mut soket = SmartSoket::new(0xDEADBEAF);
    soket.serv_runner("127.0.0.1:55331")
}
