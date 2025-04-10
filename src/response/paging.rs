//! This module provides functionality to extract pagination information from a Namecheap API response.
//!

use serde_json::{ Value, Map };
use tracing::warn;

/// Extracts pagination information from a Namecheap API response
///
/// # Parameters
///
/// - `paging`: Optional reference to a JSON Map containing paging information
///
/// # Returns
///
/// A tuple containing (current_page, page_size, total_items, total_pages)
pub fn extract_pagination_info(paging: Option<&Map<String, Value>>) -> (i64, i64, i64, i64) {
    // Default pagination values
    let mut current_page: i64 = 1;
    let mut page_size: i64 = 21;
    let mut total_items: i64 = 0;

    // Extract values if Paging exists
    if let Some(paging_obj) = paging {
        if
            let Some(current) = paging_obj
                .get("CurrentPage")
                .and_then(|v| v.get("$text"))
                .and_then(|v| v.as_str())
        {
            current_page = current.parse::<i64>().unwrap_or(1);
        }

        if
            let Some(size) = paging_obj
                .get("PageSize")
                .and_then(|v| v.get("$text"))
                .and_then(|v| v.as_str())
        {
            page_size = size.parse::<i64>().unwrap_or(21);
        }

        if
            let Some(total) = paging_obj
                .get("TotalItems")
                .and_then(|v| v.get("$text"))
                .and_then(|v| v.as_str())
        {
            total_items = total.parse::<i64>().unwrap_or(0);
        }
    } else {
        warn!("Paging information not found in response");
    }

    // Calculate total pages based on total items and page size
    let total_pages: i64 = if page_size > 0 {
        (total_items + page_size - 1) / page_size
    } else {
        0
    };

    (current_page, page_size, total_items, total_pages)
}
