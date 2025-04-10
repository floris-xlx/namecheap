//! ### `domains.dns.getList` Implementation
//!
//! This module provides the implementation for the `domains.dns.getList` method of the NameCheap API.
//!
//! It retrieves a list of DNS servers associated with a domain.
//!

use serde_json::{ Value, json };
use std::error::Error;
use tracing::{ info, error };

// crate imports
use crate::NameCheapClient;
use crate::utils::request_builder::Request;

impl NameCheapClient {
    /// - `domains.dns.getList`: Gets a list of DNS servers for the specified domain
    /// Gets a list of DNS servers associated with the requested domain
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
    ///     let dns_list = client.domains_dns_get_list("domain", "com").await.unwrap();
    ///     println!("DNS Servers: {:?}", dns_list);
    /// }
    /// ```
    pub async fn domains_dns_get_list(
        &self,
        sld: &str,
        tld: &str
    ) -> Result<Value, Box<dyn Error>> {
        let command: String = "namecheap.domains.dns.getList".to_string();
        let params: Value = json!({
            "SLD": sld,
            "TLD": tld
        });

        let response: Value = Request::new(
            self.clone(),
            command,
            Some(1),
            None,
            Some(params)
        ).send().await?;

        // Extract DNS servers from the response
        if let Some(api_response) = response.get("ApiResponse") {
            if let Some(command_response) = api_response.get("CommandResponse") {
                if
                    let Some(domain_dns_get_list_result) =
                        command_response.get("DomainDNSGetListResult")
                {
                    if let Some(nameservers) = domain_dns_get_list_result.get("Nameserver") {
                        return Ok(nameservers.clone());
                    }
                }
            }
        }

        error!("Failed to retrieve DNS server list");
        Err("Failed to retrieve DNS server list".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use serde_json::json;

    #[tokio::test]
    async fn test_domains_dns_get_list() {
        dotenv().ok();

        let client: Result<NameCheapClient, Box<dyn Error>> = NameCheapClient::new_from_env();
        let client: NameCheapClient = client.unwrap();

        let dns_list: Value = client.domains_dns_get_list("xylex", "ai").await.unwrap();

        // Expected DNS list
        let expected_dns_list: Value =
            json!([
            {"$text": "dns1.registrar-servers.com"},
            {"$text": "dns2.registrar-servers.com"}
        ]);

        // Basic validation
        assert_eq!(dns_list, expected_dns_list);
    }
}
