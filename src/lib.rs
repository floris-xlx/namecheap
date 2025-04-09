
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
pub struct NameCheapClient {
    pub api_user: String,
    pub api_key: String,
    pub client_ip: String,
    pub user_name: String,
    pub production: bool,
    pub api_url: Option<String>,
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
