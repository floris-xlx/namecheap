// utils
use namecheap::NameCheapClient;
use namecheap::utils::tracer::init_tracing;
use namecheap::domains_dns::set_hosts::HostRequest;

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
    // let tld_list = client.domains_get_tld_list().await.unwrap();
    // let get_list: Value = client.domains_dns_get_list("xylex", "ai").await.unwrap();
    let domains_get_hosts: Value = client
        .domains_dns_get_hosts("hitomi-tanaka", "com").await
        .unwrap();
    let new_host: HostRequest = HostRequest::new(
        "api".to_string(),
        "A".to_string(),
        "65.108.104.231".to_string(),
        Some("10".to_string()),
        Some("FWD".to_string()),
        Some("3600".to_string()),
        None,
        None
    );

    let domains_set_hosts: Value = client
        .domains_dns_set_hosts("hitomi-tanaka", "com", vec![new_host]).await
        .unwrap();

    info!("domains_set_hosts: {:#?}", domains_set_hosts);
    info!("domains_get_hosts: {:#?}", domains_get_hosts);

    // Example usage of the client
}
