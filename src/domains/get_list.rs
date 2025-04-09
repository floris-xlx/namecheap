//! ### `domains.getList` Implementation
//! 
//! This module provides the implementation for the `domains.getList` method of the NameCheap API.
//! 
//! It retrieves a list of domains associated with the user's account.
//! 

// crate imports
use crate::NameCheapClient;
use crate::Domain;
use crate::utils::request_builder::Request;



impl NameCheapClient {
    /// Gets a list of domains for the specified user
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
    ///     let domains = client.get_list().await.unwrap();
    ///     println!("Domains: {:?}", domains);
    /// }
    /// ```
    ///
    /// # Example Result
    ///
    /// ```
    /// Domains: [
    ///     Domain {
    ///         id: 11111111,
    ///         name: "xylex.ai",
    ///         user: "florisskx",
    ///         created: "07/14/2024",
    ///         expires: "07/14/2025",
    ///         is_expired: false,
    ///         is_locked: false,
    ///         auto_renew: false,
    ///         whois_guard: true,
    ///         is_premium: false,
    ///         is_our_dns: true,
    ///     }
    /// ]
    /// ```
    pub async fn domains_get_list(&self) -> Result<Vec<Domain>, Box<dyn std::error::Error>> {
        let command: String = "namecheap.domains.getList".to_string();
        let response: serde_json::Value = Request::new(self.clone(), command).send().await?;

        // Extract domains from the response
        if let Some(api_response) = response.get("ApiResponse") {
            if let Some(command_response) = api_response.get("CommandResponse") {
                if let Some(result) = command_response.get("DomainGetListResult") {
                    if let Some(domains) = result.get("Domain") {
                        if let Some(domains_array) = domains.as_array() {
                            let mut domain_list: Vec<Domain> = Vec::new();

                            for domain in domains_array {
                                let id: i64 = domain
                                    .get("id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("0")
                                    .parse::<i64>()
                                    .unwrap_or(0);
                                let name: String = domain
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let user: String = domain
                                    .get("user")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let created: String = domain
                                    .get("created")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let expires: String = domain
                                    .get("expires")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let is_expired: bool =
                                    domain
                                        .get("is_expired")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("false") == "true";
                                let is_locked: bool =
                                    domain
                                        .get("is_locked")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("false") == "true";
                                let auto_renew: bool =
                                    domain
                                        .get("auto_renew")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("false") == "true";
                                let whois_guard: bool =
                                    domain
                                        .get("whois_guard")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("NOTPRESENT") == "ENABLED";
                                let is_premium: bool =
                                    domain
                                        .get("is_premium")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("false") == "true";
                                let is_our_dns: bool =
                                    domain
                                        .get("is_our_dns")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("false") == "true";

                                domain_list.push(Domain {
                                    id,
                                    name,
                                    user,
                                    created,
                                    expires,
                                    is_expired,
                                    is_locked,
                                    auto_renew,
                                    whois_guard,
                                    is_premium,
                                    is_our_dns,
                                });
                            }

                            return Ok(domain_list);
                        }
                    }
                }
            }
        }

        Ok(Vec::new())
    }
}
