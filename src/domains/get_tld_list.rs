//! ### `domains.getTldList` Implementation
//!
//! This module provides the implementation for the `domains.getTldList` method of the NameCheap API.
//!
//! It retrieves a list of supported TLDs.
//!

use serde_json::{ Value, json };
use std::error::Error;
use tracing::{ info, error };

// crate imports
use crate::NameCheapClient;
use crate::utils::request_builder::Request;

impl NameCheapClient {
    /// Gets a list of supported TLDs
    ///
    /// ## Warning
    /// This gives an absolute massive response, so be careful when using it.
    ///
    /// # Example
    ///
    /// ```rust
    /// use namecheap::NameCheapClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = NameCheapClient::new(
    ///         "api_user".to_string(),
    ///         "api_key".to_string(),
    ///         "client_ip".to_string(),
    ///         "user_name".to_string(),
    ///         false
    ///     );
    ///
    ///     let tlds = client.domains_get_tld_list().await.unwrap();
    ///     println!("Supported TLDs: {:?}", tlds);
    /// }
    /// ```
    pub async fn domains_get_tld_list(&self) -> Result<Value, Box<dyn Error>> {
        let command: String = "namecheap.domains.getTldList".to_string();

        let response: Value = Request::new(self.clone(), command, None, None).send().await?;

        info!("Response: {:#?}", response);
        // Extract TLDs from the response
        if let Some(api_response) = response.get("ApiResponse") {
            if let Some(command_response) = api_response.get("CommandResponse") {
                if let Some(result) = command_response.get("Tlds") {
                    return Ok(result.clone());
                }
            }
        }

        error!("Failed to retrieve TLD list");
        Err("Failed to retrieve TLD list".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_domains_get_tld_list() {
        dotenv().ok();

        let client: Result<NameCheapClient, Box<dyn Error>> = NameCheapClient::new_from_env();
        let client: NameCheapClient = client.unwrap();

        let contacts: Value = client.domains_get_tld_list().await.unwrap();

        // Basic validation
        assert!(contacts.get("Tld").is_some());
    }
}
