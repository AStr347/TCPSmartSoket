use CLIClient::CLIClient;
use TCPClient::TCPClient;

fn main() {
    let mut cli = CLIClient {};
    cli.client_runner("127.0.0.1:55331");
}
