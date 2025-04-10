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

### NameCheap API Coverage


#### `domains`
- [x] **namecheap.domains.getList**: Retrieve a list of domains associated with your account.
- [x] **namecheap.domains.getContacts**: Get contact information for a specific domain.
- [ ] **namecheap.domains.create**: Register a new domain.
- [x] **namecheap.domains.getTldList**: Retrieve a list of supported TLDs.
- [ ] **namecheap.domains.setContacts**: Update contact information for a domain.
- [ ] **namecheap.domains.check**: Check the availability of a domain.
- [ ] **namecheap.domains.reactivate**: Reactivate an expired domain.
- [ ] **namecheap.domains.renew**: Renew a domain registration.
- [ ] **namecheap.domains.getRegistrarLock**: Get the registrar lock status of a domain.
- [ ] **namecheap.domains.setRegistrarLock**: Set the registrar lock status of a domain.
- [ ] **namecheap.domains.getInfo**: Retrieve detailed information about a domain.

#### `domains.dns`
- [ ] **namecheap.domains.dns.setDefault**: Set the DNS settings of a domain to the default Namecheap settings.
- [ ] **namecheap.domains.dns.setCustom**: Set custom DNS settings for a domain.
- [x] **namecheap.domains.dns.getList**: Retrieve a list of DNS servers associated with a domain.
- [ ] **namecheap.domains.dns.getHosts**: Retrieve the host records for a domain.
- [ ] **namecheap.domains.dns.getEmailForwarding**: Get the email forwarding settings for a domain.
- [ ] **namecheap.domains.dns.setEmailForwarding**: Set the email forwarding settings for a domain.
- [ ] **namecheap.domains.dns.setHosts**: Set the host records for a domain.

#### `domains.ns`
- [ ] **namecheap.domains.ns.create**: Create a new nameserver under your domain.
- [ ] **namecheap.domains.ns.delete**: Delete an existing nameserver associated with your domain.
- [ ] **namecheap.domains.ns.getInfo**: Retrieve detailed information about a specific nameserver.
- [ ] **namecheap.domains.ns.update**: Update the details of an existing nameserver.

#### `domains.transfer`
- [ ] **namecheap.domains.transfer.create**: Initiate a domain transfer to Namecheap.
- [ ] **namecheap.domains.transfer.getStatus**: Retrieve the status of a domain transfer.
- [ ] **namecheap.domains.transfer.updateStatus**: Update the status of a domain transfer.
- [ ] **namecheap.domains.transfer.getList**: Retrieve a list of domain transfers associated with your account.

#### `ssl`
- [ ] **namecheap.ssl.create**: Create a new SSL certificate.
- [ ] **namecheap.ssl.getList**: Retrieve a list of SSL certificates associated with your account.
- [ ] **namecheap.ssl.parseCSR**: Parse a Certificate Signing Request (CSR).
- [ ] **namecheap.ssl.getApproverEmailList**: Get a list of approver email addresses for a domain.
- [ ] **namecheap.ssl.activate**: Activate an SSL certificate.
- [ ] **namecheap.ssl.resendApproverEmail**: Resend the approver email for an SSL certificate.
- [ ] **namecheap.ssl.getInfo**: Retrieve detailed information about an SSL certificate.
- [ ] **namecheap.ssl.renew**: Renew an existing SSL certificate.
- [ ] **namecheap.ssl.reissue**: Reissue an SSL certificate.
- [ ] **namecheap.ssl.resendfulfillmentemail**: Resend the fulfillment email for an SSL certificate.
- [ ] **namecheap.ssl.purchasemoresans**: Purchase additional Subject Alternative Names (SANs) for an SSL certificate.
- [ ] **namecheap.ssl.revokecertificate**: Revoke an SSL certificate.
- [ ] **namecheap.ssl.editDCVMethod**: Edit the Domain Control Validation (DCV) method for an SSL certificate.

#### `users`
- [ ] **namecheap.users.getPricing**: Retrieve pricing information for various services.
- [ ] **namecheap.users.getBalances**: Get the current balance of your Namecheap account.
- [ ] **namecheap.users.changePassword**: Change the password for your Namecheap account.
- [ ] **namecheap.users.update**: Update user information for your Namecheap account.
- [ ] **namecheap.users.createaddfundsrequest**: Create a request to add funds to your Namecheap account.
- [ ] **namecheap.users.getAddFundsStatus**: Retrieve the status of an add funds request.
- [ ] **namecheap.users.create**: Create a new user under your Namecheap account.
- [ ] **namecheap.users.login**: Log in to your Namecheap account.
- [ ] **namecheap.users.resetPassword**: Reset the password for your Namecheap account.

#### `users.address`
- [ ] **namecheap.users.address.create**: Create a new address for a user.
- [ ] **namecheap.users.address.delete**: Delete an existing address associated with a user.
- [ ] **namecheap.users.address.getInfo**: Retrieve detailed information about a specific address.
- [ ] **namecheap.users.address.getList**: Retrieve a list of addresses associated with a user.
- [ ] **namecheap.users.address.setDefault**: Set a specific address as the default for a user.
- [ ] **namecheap.users.address.update**: Update the details of an existing address.

#### `domainprivacy`
- [ ] **namecheap.domainprivacy.changeemailaddress**: Change the email address associated with domain privacy.
- [ ] **namecheap.domainprivacy.enable**: Enable domain privacy for a domain.
- [ ] **namecheap.domainprivacy.disable**: Disable domain privacy for a domain.
- [ ] **namecheap.domainprivacy.getList**: Retrieve a list of domains with privacy protection.
- [ ] **namecheap.domainprivacy.renew**: Renew domain privacy for a domain.
