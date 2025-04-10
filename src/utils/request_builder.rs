//! ## RequestBuilder Module
//! This module provides the `RequestBuilder` struct, which is used to construct
//! requests to the NameCheap API. It allows you to set up the necessary parameters,
//! build the URL for the API request, and send the request.

use reqwest::{ Client, Response, Method, RequestBuilder };
use tracing::info;
use serde::{ Serialize, Deserialize };
use anyhow::{ Result, anyhow };
use serde_json::Value;

// crate imports
use crate::{ NameCheapClient, NAMECHEAP_API_URL, NAMECHEAP_SANDBOX_API_URL };
use crate::utils::xml_parser::parse_xml_to_json;

/// A builder for constructing requests to the NameCheap API.
///
/// The `RequestBuilder` struct is used to create and configure requests
/// to be sent to the NameCheap API. It holds the necessary client
/// information and the specific command to be executed.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(PartialEq, Eq, Hash)]
pub struct Request {
    /// The `NameCheapClient` instance containing API credentials and configuration.
    client: NameCheapClient,
    /// The specific API command to be executed.
    command: String,
    /// The page number for paginated results (optional).
    page: Option<i64>,
    /// Domain name (optional).
    domain_name: Option<String>,
    /// Domain ID (optional).
    domain_id: Option<i64>,
}

impl Request {
    /// Creates a new `RequestBuilder` instance.
    ///
    /// # Parameters
    ///
    /// - `client`: A `NameCheapClient` instance with the necessary credentials.
    /// - `command`: A `String` representing the API command to be executed.
    /// - `page`: An optional page number for paginated results.
    ///
    /// # Returns
    ///
    /// A new `RequestBuilder` instance.
    pub fn new(
        client: NameCheapClient,
        command: String,
        page: Option<i64>,
        domain_name: Option<String>
    ) -> Self {
        Request {
            client,
            command,
            page,
            domain_name,
            domain_id: None,
        }
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

        // Start with the base parameters
        let mut url = format!(
            "{}/xml.response?ApiUser={}&ApiKey={}&UserName={}&Command={}&ClientIp={}",
            base_url,
            self.client.api_user,
            self.client.api_key,
            self.client.user_name,
            self.command,
            self.client.client_ip
        );

        // Add optional parameters if they exist
        if let Some(page) = self.page {
            url.push_str(&format!("&Page={}", page));
        }

        if let Some(ref domain_name) = self.domain_name {
            url.push_str(&format!("&DomainName={}", domain_name));
        }

        if let Some(domain_id) = self.domain_id {
            url.push_str(&format!("&DomainID={}", domain_id));
        }

        url
    }

    /// Sends the API request and returns the response.
    ///
    /// This method sends a GET request to the constructed URL and returns the
    /// response from the NameCheap API.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Value` if successful, or an `Error` if the request fails.
    pub async fn send(&self) -> Result<Value> {
        let url: String = self.build_url();
        info!("Sending request to URL: {:#?}", url);

        let client: Client = Client::new();
        let request: RequestBuilder = client
            .request(Method::GET, &url)
            .header("Accept", "application/xml")
            .header("Content-Type", "application/xml");

        let response: Response = request.send().await?;

        // Ensure we're receiving XML
        if let Some(content_type) = response.headers().get("Content-Type") {
            if !content_type.to_str().unwrap_or("").contains("xml") {
                return Err(anyhow!("Response is not XML"));
            }
        }

        // Get the response body as a string
        let response_text: String = response.text().await?;

        // Parse XML to JSON
        let json_value: Value = parse_xml_to_json(&response_text)?;
        Ok(json_value)
    }

    /// Sets the domain name for the request.
    ///
    /// # Parameters
    ///
    /// - `domain_name`: The domain name to set.
    ///
    /// # Returns
    ///
    /// The modified `Request` instance for method chaining.
    pub fn with_domain_name(mut self, domain_name: String) -> Self {
        self.domain_name = Some(domain_name);
        self
    }

    /// Sets the domain ID for the request.
    ///
    /// # Parameters
    ///
    /// - `domain_id`: The domain ID to set.
    ///
    /// # Returns
    ///
    /// The modified `Request` instance for method chaining.
    pub fn with_domain_id(mut self, domain_id: i64) -> Self {
        self.domain_id = Some(domain_id);
        self
    }
}
