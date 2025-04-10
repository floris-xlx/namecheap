//! ### `domains.getContacts` Implementation
//!
//! This module provides the implementation for the `domains.getContacts` method of the NameCheap API.
//!
//! It retrieves the contact information for a specified domain.
//!

use serde_json::{ Value, json };
use std::error::Error;
use tracing::{ info, error };

// crate imports
use crate::{ NameCheapClient, Contact };
use crate::utils::request_builder::Request;
use crate::response::parse_value::parse_string;

impl NameCheapClient {
    /// Gets contact information for the specified domain
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
    ///     let contacts = client.domains_get_contacts("example.com").await.unwrap();
    ///     println!("Domain Contacts: {:?}", contacts);
    /// }
    /// ```
    pub async fn domains_get_contacts(&self, domain_name: &str) -> Result<Value, Box<dyn Error>> {
        let command: String = "namecheap.domains.getContacts".to_string();

        let response: Value = Request::new(
            self.clone(),
            command,
            None,
            Some(domain_name.to_string())
        ).send().await?;

        info!("Response: {:#?}", response);
        // Extract contacts from the response
        if let Some(api_response) = response.get("ApiResponse") {
            if let Some(command_response) = api_response.get("CommandResponse") {
                if let Some(result) = command_response.get("DomainContactsResult") {
                    let mut contacts: Value = json!({});
                    let mut whois_guard_contacts: Value = json!({});
                    let has_whois_guard: bool = result.get("WhoisGuardContact").is_some();

                    // Process each contact type
                    for contact_type in &["Registrant", "Tech", "Admin", "AuxBilling"] {
                        if let Some(contact_info) = result.get(contact_type) {
                            // Create a Contact struct for each contact type
                            let contact: Contact = Contact {
                                type_: contact_type.to_string(),
                                first_name: parse_string(contact_info, "FirstName", ""),
                                last_name: parse_string(contact_info, "LastName", ""),
                                address_1: parse_string(contact_info, "Address1", ""),
                                address_2: parse_string(contact_info, "Address2", ""),
                                city: parse_string(contact_info, "City", ""),
                                state_province: parse_string(contact_info, "StateProvince", ""),
                                state_province_choice: parse_string(
                                    contact_info,
                                    "StateProvinceChoice",
                                    ""
                                ),
                                postal_code: parse_string(contact_info, "PostalCode", ""),
                                country: parse_string(contact_info, "Country", ""),
                                phone: parse_string(contact_info, "Phone", ""),
                                phone_ext: parse_string(contact_info, "PhoneExt", ""),
                                fax: parse_string(contact_info, "Fax", ""),
                                email_address: parse_string(contact_info, "EmailAddress", ""),
                                organization_name: parse_string(
                                    contact_info,
                                    "OrganizationName",
                                    ""
                                ),
                                job_title: parse_string(contact_info, "JobTitle", ""),
                                read_only: parse_string(contact_info, "read_only", "false") ==
                                "true",
                            };

                            contacts[contact_type.to_lowercase()] = json!(contact);
                        }

                        // Process WhoisGuard contacts if available
                        if has_whois_guard {
                            if let Some(whois_guard) = result.get("WhoisGuardContact") {
                                if let Some(whois_contact_info) = whois_guard.get(contact_type) {
                                    let whois_contact: Contact = Contact {
                                        type_: contact_type.to_string(),
                                        first_name: parse_string(
                                            whois_contact_info,
                                            "FirstName",
                                            ""
                                        ),
                                        last_name: parse_string(whois_contact_info, "LastName", ""),
                                        address_1: parse_string(whois_contact_info, "Address1", ""),
                                        address_2: parse_string(whois_contact_info, "Address2", ""),
                                        city: parse_string(whois_contact_info, "City", ""),
                                        state_province: parse_string(
                                            whois_contact_info,
                                            "StateProvince",
                                            ""
                                        ),
                                        state_province_choice: parse_string(
                                            whois_contact_info,
                                            "StateProvinceChoice",
                                            ""
                                        ),
                                        postal_code: parse_string(
                                            whois_contact_info,
                                            "PostalCode",
                                            ""
                                        ),
                                        country: parse_string(whois_contact_info, "Country", ""),
                                        phone: parse_string(whois_contact_info, "Phone", ""),
                                        phone_ext: parse_string(whois_contact_info, "PhoneExt", ""),
                                        fax: parse_string(whois_contact_info, "Fax", ""),
                                        email_address: parse_string(
                                            whois_contact_info,
                                            "EmailAddress",
                                            ""
                                        ),
                                        organization_name: parse_string(
                                            whois_contact_info,
                                            "OrganizationName",
                                            ""
                                        ),
                                        job_title: parse_string(whois_contact_info, "JobTitle", ""),
                                        read_only: parse_string(
                                            whois_contact_info,
                                            "read_only",
                                            "false"
                                        ) == "true",
                                    };

                                    whois_guard_contacts[contact_type.to_lowercase()] =
                                        json!(whois_contact);
                                }
                            }
                        }
                    }

                    // Add domain information and WhoisGuard contacts if available
                    let mut result_json: Value =
                        json!({
                        "contacts": contacts,
                        "domain": parse_string(result, "domain", ""),
                        "domain_id": parse_string(result, "domainnameid", "")
                    });

                    if has_whois_guard {
                        result_json["whois_guard_contacts"] = whois_guard_contacts;
                    }

                    return Ok(result_json);
                }
            }
        }

        // Return if no contacts found
        error!("Failed to extract contact information for domain: {}", domain_name);
        Ok(
            json!({
            "error": true,
            "message": format!("Failed to extract contact information for domain: {}", domain_name)
        })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_domains_get_contacts() {
        dotenv().ok();

        let client: Result<NameCheapClient, Box<dyn Error>> = NameCheapClient::new_from_env();
        let client: NameCheapClient = client.unwrap();

        let contacts: Value = client.domains_get_contacts("xylex.ai").await.unwrap();
        info!("contacts: {:#?}", contacts);

        // Basic validation
        assert!(contacts.get("contacts").is_some());
        assert!(contacts.get("domain").is_some());
        assert!(contacts.get("domain_id").is_some());
        assert!(contacts.get("whois_guard_contacts").is_some());
    }
}
