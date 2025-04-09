//! ## RequestBuilder Module
//! This module provides the `RequestBuilder` struct, which is used to construct
//! requests to the NameCheap API. It allows you to set up the necessary parameters
//! and build the URL for the API request.
//!

use crate::{ NameCheapClient, NAMECHEAP_API_URL, NAMECHEAP_SANDBOX_API_URL };

/// A builder for constructing requests to the NameCheap API.
///
/// The `RequestBuilder` struct is used to create and configure requests
/// to be sent to the NameCheap API. It holds the necessary client
/// information and the specific command to be executed.
pub struct RequestBuilder {
    /// The `NameCheapClient` instance containing API credentials and configuration.
    client: NameCheapClient,
    /// The specific API command to be executed.
    command: String,
}

impl RequestBuilder {
    /// Creates a new `RequestBuilder` instance.
    ///
    /// # Parameters
    ///
    /// - `client`: A `NameCheapClient` instance with the necessary credentials.
    /// - `command`: A `String` representing the API command to be executed.
    ///
    /// # Returns
    ///
    /// A new `RequestBuilder` instance.
    pub fn new(client: NameCheapClient, command: String) -> Self {
        RequestBuilder { client, command }
    }

    /// Builds the URL for the API request.
    ///
    /// This method constructs the full URL for the API request based on the
    /// client's configuration and the specified command.
    ///
    /// # Returns
    ///
    /// A `String` containing the full URL for the API request.
    pub fn build_url(&self) -> String {
        let base_url = if self.client.production {
            NAMECHEAP_API_URL
        } else {
            NAMECHEAP_SANDBOX_API_URL
        };

        // Construct the URL using the base URL and the client's credentials
        format!(
            "{}/xml.response?ApiUser={}&ApiKey={}&UserName={}&Command={}&ClientIp={}",
            base_url,
            self.client.api_user,
            self.client.api_key,
            self.client.user_name,
            self.command,
            self.client.client_ip
        )
    }
}
