
use serde::Deserialize;
use serde::Serialize;

use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

pub mod utils;
pub mod domains;

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
}
