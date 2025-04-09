// utils
use namecheap::NameCheapClient;
use namecheap::utils::tracer::init_tracing;
use namecheap::utils::request_builder::Request;
use namecheap::Domain;

use dotenv::dotenv;
use std::env::var;
use tracing::{ error, info };
use reqwest::Response;



#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");
    init_tracing();

    let api_key: String = var("NAMECHEAP_API_KEY").expect("NAMECHEAP_API_KEY must be set");
    let api_username: String = var("NAMECHEAP_USER_NAME").expect("NAMECHEAP_USER_NAME must be set");

    let client_ip: String = var("NAMECHEAP_CLIENT_IP").expect("NAMECHEAP_CLIENT_IP must be set");
    let user_name: String = var("NAMECHEAP_USER_NAME").expect("NAMECHEAP_USER_NAME must be set");
    let production: bool = var("NAMECHEAP_PRODUCTION")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .expect("NAMECHEAP_PRODUCTION must be a boolean");

    let client: NameCheapClient = NameCheapClient::new(
        api_username,
        api_key,
        client_ip,
        user_name,
        production
    );
    info!("NameCheapClient created: {:#?}", client);

    let domains: Vec<namecheap::Domain> = client.domains_get_list().await.unwrap();
    info!("Domains: {:#?}", domains);



    // Example usage of the client
}
