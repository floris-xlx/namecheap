

// utils
use namecheap::utils::tracer::init_tracing;
use namecheap::NameCheapClient;



#[tokio::main]
async fn main() {
    println!("Hello, world!");
    init_tracing();
}

