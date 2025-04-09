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
}

impl Request {
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
        Request { client, command }
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

    /// Sends the API request and returns the response.
    ///
    /// This method sends a GET request to the constructed URL and returns the
    /// response from the NameCheap API.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Response` if successful, or an `Error` if the request fails.
    pub async fn send(&self) -> Result<Value> {
        let url: String = self.build_url();
        info!("Sending request to URL: {}", url);
        let client: Client = Client::new();
        let request: RequestBuilder = client
            .request(Method::GET, &url)
            .header("Accept", "application/xml")
            .header("Content-Type", "application/xml");

        let response: Response = request.send().await?;
        info!("Response: {:#?}", response);

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
}
