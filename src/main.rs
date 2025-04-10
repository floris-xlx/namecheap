// utils
use namecheap::NameCheapClient;
use namecheap::utils::tracer::init_tracing;

use dotenv::dotenv;
use std::env::var;
use serde_json::Value;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let client: NameCheapClient = NameCheapClient::new(
        var("NAMECHEAP_USER_NAME").expect("NAMECHEAP_USER_NAME must be set"),
        var("NAMECHEAP_API_KEY").expect("NAMECHEAP_API_KEY must be set"),
        var("NAMECHEAP_CLIENT_IP").expect("NAMECHEAP_CLIENT_IP must be set"),
        var("NAMECHEAP_USER_NAME").expect("NAMECHEAP_USER_NAME must be set"),
        var("NAMECHEAP_PRODUCTION")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("NAMECHEAP_PRODUCTION must be a boolean")
    );
    // let domains: Value = client.domains_get_list(1).await.unwrap();
    // let contacts = client.domains_get_contacts("xylex.ai").await.unwrap();
    let tld_list = client.domains_get_tld_list().await.unwrap();
    // let get_list: Value = client.domains_dns_get_list("xylex", "ai").await.unwrap();



    info!("get_list: {:#?}", tld_list);

    // Example usage of the client
}
