//! This module provides utility functions for parsing values from Namecheap API responses.
//!
//! It contains functions to safely extract and convert values from JSON responses.

use serde_json::Value;

/// Parses a string value from a JSON object
///
/// # Parameters
///
/// - `json`: The JSON object to extract from
/// - `key`: The key to look for in the JSON object
/// - `default`: The default value to return if the key is not found or not a string
///
/// # Returns
///
/// The string value or the default if not found
pub fn parse_string(json: &Value, key: &str, default: &str) -> String {
    json.get(key)
        .and_then(|v| {
            if let Some(text) = v.get("$text") {
                text.as_str()
            } else {
                v.as_str()
            }
        })
        .unwrap_or(default)
        .to_string()
}

/// Parses a boolean value from a JSON object
///
/// # Parameters
///
/// - `json`: The JSON object to extract from
/// - `key`: The key to look for in the JSON object
/// - `default`: The default value to return if the key is not found
/// - `true_value`: The string value that represents true (default is "true")
///
/// # Returns
///
/// The boolean value or the default if not found
pub fn parse_bool(json: &Value, key: &str, default: &str, true_value: &str) -> bool {
    json
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default) == true_value
}

/// Parses an integer value from a JSON object
///
/// # Parameters
///
/// - `json`: The JSON object to extract from
/// - `key`: The key to look for in the JSON object
/// - `default`: The default value to return if the key is not found or parsing fails
///
/// # Returns
///
/// The integer value or the default if not found or parsing fails
pub fn parse_i64(json: &Value, key: &str, default: i64) -> i64 {
    json.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("0")
        .parse::<i64>()
        .unwrap_or(default)
}
