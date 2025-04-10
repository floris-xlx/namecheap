//! ## Domains API
//! The Domains API provides a set of methods to manage domains, including retrieving a list of domains, checking domain availability, and more.
//!
//! ### Available Methods
//! - `namecheap.domains.getList`: Retrieve a list of domains associated with your account.
//! - `namecheap.domains.getContacts`: Get contact information for a specific domain.
//! - `namecheap.domains.create`: Register a new domain.
//! - `namecheap.domains.getTldList`: Retrieve a list of supported TLDs.
//! - `namecheap.domains.setContacts`: Update contact information for a domain.
//! - `namecheap.domains.check`: Check the availability of a domain.
//! - `namecheap.domains.reactivate`: Reactivate an expired domain.
//! - `namecheap.domains.renew`: Renew a domain registration.
//! - `namecheap.domains.getRegistrarLock`: Get the registrar lock status of a domain.
//! - `namecheap.domains.setRegistrarLock`: Set the registrar lock status of a domain.
//! - `namecheap.domains.getInfo`: Retrieve detailed information about a domain.
//!
//!

// crate imports
use crate::NameCheapClient;
use crate::Domain;
use crate::utils::request_builder::Request;

/// - **domains.getList**
pub mod get_list;
/// - **domains.getContacts**
pub mod get_contacts;