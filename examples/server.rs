use SmartSoket::SmartSoket;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let soket = SmartSoket::new(0xDEADBEAF);
    soket.serv_runner("127.0.0.1:55331").await;
}
