//! ### `domains.getList` Implementation
//!
//! This module provides the implementation for the `domains.getList` method of the NameCheap API.
//!
//! It retrieves a list of domains associated with the user's account.
//!
//!
use serde_json::{ Value, json, Map };
use tracing::{ info, error, warn };
use std::error::Error;

// crate imports
use crate::{ NameCheapClient, Domain };
use crate::utils::request_builder::Request;
use crate::response::paging::extract_pagination_info;
use crate::response::parse_value::{ parse_string, parse_bool, parse_i64 };

impl NameCheapClient {
    /// - `domains.getList`: Gets a list of domains for the specified user
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
    pub async fn domains_get_list(&self, page: i64) -> Result<Value, Box<dyn Error>> {
        let command: String = "namecheap.domains.getList".to_string();
        let page: i64 = page.max(1);
        let page: Option<i64> = Some(page);

        let response: Value = Request::new(self.clone(), command, page, None, None).send().await?;

        // Extract domains from the response
        if let Some(api_response) = response.get("ApiResponse") {
            if let Some(command_response) = api_response.get("CommandResponse") {
                let paging: Option<&Map<String, Value>> = command_response
                    .get("Paging")
                    .and_then(|p| p.as_object());

                // Extract pagination information using the utility function
                let (current_page, page_size, total_items, total_pages) =
                    extract_pagination_info(paging);
                if let Some(result) = command_response.get("DomainGetListResult") {
                    // Extract pagination information
                    // Extract pagination information from the Paging object

                    if let Some(domains) = result.get("Domain") {
                        if let Some(domains_array) = domains.as_array() {
                            let mut domain_list: Vec<Domain> = Vec::new();

                            for domain in domains_array {
                                let id: i64 = parse_i64(domain, "id", 0);
                                let name: String = parse_string(domain, "name", "");
                                let user: String = parse_string(domain, "user", "");
                                let created: String = parse_string(domain, "created", "");
                                let expires: String = parse_string(domain, "expires", "");
                                let is_expired: bool = parse_bool(
                                    domain,
                                    "is_expired",
                                    "false",
                                    "true"
                                );
                                let is_locked: bool = parse_bool(
                                    domain,
                                    "is_locked",
                                    "false",
                                    "true"
                                );
                                let auto_renew: bool = parse_bool(
                                    domain,
                                    "auto_renew",
                                    "false",
                                    "true"
                                );
                                let whois_guard: bool = parse_bool(
                                    domain,
                                    "whois_guard",
                                    "NOTPRESENT",
                                    "ENABLED"
                                );
                                let is_premium: bool = parse_bool(
                                    domain,
                                    "is_premium",
                                    "false",
                                    "true"
                                );
                                let is_our_dns: bool = parse_bool(
                                    domain,
                                    "is_our_dns",
                                    "false",
                                    "true"
                                );

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

                            // Create a Value object with domains and pagination info
                            let result_value: Value =
                                json!({
                                "domains": domain_list,
                                "pagination": {
                                    "currentPage": current_page,
                                    "totalPages": total_pages
                                }
                            });

                            return Ok(result_value);
                        }
                    }
                }
            }
        }

        // Return empty result with default pagination
        let empty_result: Value =
            json!({
            "domains": [],
            "pagination": {
                "currentPage": 1,
                "totalPages": 1
            }
        });

        Ok(empty_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use tracing::info;

    #[tokio::test]
    async fn test_domains_get_list() {
        dotenv().ok();

        let client: Result<NameCheapClient, Box<dyn Error>> = NameCheapClient::new_from_env();
        let client: NameCheapClient = client.unwrap();

        let domains: Value = client.domains_get_list(1).await.unwrap();
        info!("Domains: {:#?}", domains);

        // Basic validation
        assert!(domains.get("domains").is_some());
        assert!(domains.get("pagination").is_some());
    }
}
