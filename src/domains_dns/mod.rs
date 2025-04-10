//! ## Domains DNS API
//! The Domains DNS API provides a set of methods to manage DNS settings for domains, including setting default or custom DNS, retrieving DNS server lists, and managing host records and email forwarding.
//!
//! ### Available Methods
//! - `namecheap.domains.dns.setDefault`: Set the DNS settings of a domain to the default Namecheap settings.
//! - `namecheap.domains.dns.setCustom`: Set custom DNS settings for a domain.
//! - `namecheap.domains.dns.getList`: Retrieve a list of DNS servers associated with a domain.
//! - `namecheap.domains.dns.getHosts`: Retrieve the host records for a domain.
//! - `namecheap.domains.dns.getEmailForwarding`: Get the email forwarding settings for a domain.
//! - `namecheap.domains.dns.setEmailForwarding`: Set the email forwarding settings for a domain.
//! - `namecheap.domains.dns.setHosts`: Set the host records for a domain.
//!
//! These methods allow for comprehensive management of DNS configurations, ensuring that domain settings can be tailored to specific needs or reverted to default configurations as required.

pub mod get_list;