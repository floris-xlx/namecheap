
use serde::Deserialize;
use serde::Serialize;
use dotenv::dotenv;


pub mod utils;
pub mod domains;
pub mod response;
pub mod domains_dns;

pub const NAMECHEAP_API_URL: &str = "https://api.namecheap.com";
pub const NAMECHEAP_SANDBOX_API_URL: &str = "https://api.sandbox.namecheap.com";


/// ### NameCheap API Client
///
/// This struct represents a client for the NameCheap API.
///
/// It contains the necessary credentials and configuration for making API requests.
///
/// #### Fields
/// - `api_user`: The API user name.
/// - `api_key`: The API key.
/// - `client_ip`: The client IP address.
/// - `user_name`: The user name.
/// - `production`: A boolean indicating whether to use the production environment.
///
/// #### Note
/// `production` is a boolean defaulted to `false`. If set to `true`, the client will
/// use the production environment. If set to `false`, it will use the sandbox environment.
///
///
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(PartialEq, Eq, Hash)]
pub struct NameCheapClient {
    pub api_user: String,
    pub api_key: String,
    pub client_ip: String,
    pub user_name: String,
    pub production: bool,
    pub api_url: Option<String>,
}


/// ### Domain
/// 
/// This struct represents a domain object returned by the NameCheap API.
/// It contains various fields that provide information about the domain.
/// 
/// #### Fields
/// - `id`: The unique identifier for the domain.
/// - `name`: The name of the domain.
/// - `user`: The user associated with the domain.
/// - `created`: The creation date of the domain.
/// - `expires`: The expiration date of the domain.
/// - `is_expired`: A boolean indicating whether the domain is expired.
/// - `is_locked`: A boolean indicating whether the domain is locked.
/// - `auto_renew`: A boolean indicating whether auto-renew is enabled.
/// - `who_is_guard`: A boolean indicating whether WHOIS guard is enabled.
/// - `is_premium`: A boolean indicating whether the domain is premium.
/// - `is_our_dns`: A boolean indicating whether the domain uses NameCheap's DNS.
/// 
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(PartialEq, Eq, Hash)]
pub struct Domain {
    pub id: i64,
    pub name: String,
    pub user: String,
    pub created: String,
    pub expires: String,
    pub is_expired: bool,
    pub is_locked: bool,
    pub auto_renew: bool,
    pub whois_guard: bool,
    pub is_premium: bool,
    pub is_our_dns: bool,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(PartialEq, Eq, Hash)]
pub struct Contact {
    pub type_: String,
    pub address_1: String,
    pub address_2: String,
    pub city: String,
    pub country: String,
    pub email_address: String,
    pub fax: String,
    pub first_name: String,
    pub job_title: String,
    pub last_name: String,
    pub organization_name: String,
    pub phone: String,
    pub phone_ext: String,
    pub postal_code: String,
    pub state_province: String,
    pub state_province_choice: String,
    pub read_only: bool,
}



impl NameCheapClient {
    /// Creates a new `NameCheapClient` instance with the provided credentials and configuration.
    ///
    /// Both the `api_user` and `user_name` are your NameCheap account username.
    ///
    /// #### Parameters
    /// - `api_user`: The API user name.
    /// - `api_key`: The API key.
    /// - `client_ip`: The client IP address.
    /// - `user_name`: The user name.
    /// - `production`: A boolean indicating whether to use the production environment.
    ///
    /// #### Example
    /// ```rust
    /// let client: NameCheapClient = NameCheapClient::new(
    ///     api_username,
    ///     api_key,
    ///     client_ip,
    ///     user_name,
    ///     production
    /// );
    /// ```
    ///
    pub fn new(
        api_user: String,
        api_key: String,
        client_ip: String,
        user_name: String,
        production: bool
    ) -> Self {
        NameCheapClient {
            api_user,
            api_key,
            client_ip,
            user_name,
            production,
            api_url: if production {
                Some(NAMECHEAP_API_URL.to_string())
            } else {
                Some(NAMECHEAP_SANDBOX_API_URL.to_string())
            },
        }
    }

    /// Creates a new `NameCheapClient` instance from environment variables.
    ///
    /// This method expects the following environment variables to be set:
    /// - `NAMECHEAP_USER_NAME`: Your NameCheap account username
    /// - `NAMECHEAP_API_KEY`: Your NameCheap API key
    /// - `NAMECHEAP_CLIENT_IP`: Your client IP address
    /// - `NAMECHEAP_PRODUCTION`: Boolean indicating whether to use production environment (defaults to false)
    ///
    /// #### Example
    /// ```rust
    /// use dotenv::dotenv;
    /// 
    /// dotenv().ok(); // Load environment variables from .env file
    /// let client = NameCheapClient::new_from_env().expect("Failed to create client from environment");
    /// ```
    ///
    pub fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        use std::env::var;
        
        let user_name = var("NAMECHEAP_USER_NAME")?;
        let api_key = var("NAMECHEAP_API_KEY")?;
        let client_ip = var("NAMECHEAP_CLIENT_IP")?;
        let production = var("NAMECHEAP_PRODUCTION")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);
            
        Ok(Self::new(
            user_name.clone(),
            api_key,
            client_ip,
            user_name,
            production
        ))
    }
}
