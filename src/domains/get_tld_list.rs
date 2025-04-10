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
    /// - `domains.getTldList`: Gets a list of supported TLDs
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

        let response: Value = Request::new(self.clone(), command, None, None, None).send().await?;

        // Extract TLDs from the response
        if let Some(api_response) = response.get("ApiResponse") {
            if let Some(command_response) = api_response.get("CommandResponse") {
                if let Some(tlds) = command_response.get("Tlds") {
                    if let Some(tld_list) = tlds.get("Tld") {
                        return Ok(tld_list.clone());
                    }
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
    use std::error::Error;

    #[tokio::test]
    async fn test_domains_get_tld_list() -> Result<(), Box<dyn Error>> {

        let client: NameCheapClient = NameCheapClient::new_from_env()?;
        let tld_list: Value = client.domains_get_tld_list().await?;

        // Basic validation
        assert!(tld_list.get(0).is_some());

        Ok(())
    }
}
