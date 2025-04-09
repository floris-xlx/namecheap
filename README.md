# namecheap

## Overview

The Namecheap Rust SDK is a library designed to facilitate interaction with the Namecheap API. It provides a structured and efficient way to manage domain-related operations, such as registration, renewal, and DNS management, directly from your Rust applications.

## Features

- **API Client**: A robust client for making requests to the Namecheap API.
- **Environment Configuration**: Easily switch between production and sandbox environments.
- **Logging and Tracing**: Integrated with `tracing` for detailed logging and diagnostics.

## Getting Started

To use the Namecheap Rust SDK, you need to set up your environment variables with your Namecheap API credentials. Refer to the `.env.example` file for the required variables.

## Example

```rust
let client: NameCheapClient = NameCheapClient::new(
    api_username,
    api_key,
    client_ip,
    user_name,
    production
);
```
