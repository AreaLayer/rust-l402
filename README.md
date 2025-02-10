# Rust L402 Protocol🦀⚡

[![Bitcoin-only](https://img.shields.io/badge/bitcoin-only-FF9900?logo=bitcoin)](https://twentyone.world)
[![LN](https://img.shields.io/badge/lightning-792EE5?logo=lightning)](https://mempool.space/lightning)
[![Nostr](https://img.shields.io/badge/nostr-only-FF9900?)]((https://user-images.githubusercontent.com/99301796/223592277-34058d0e-af30-411d-8dfe-87c42dacdcf2.png))
[![crates.io](https://img.shields.io/crates/v/rust-l402)](https://crates.io/crates/rust-l402)
[![Rust](https://github.com/AreaLayer/rust-l402/actions/workflows/rust.yml/badge.svg)](https://github.com/AreaLayer/rust-l402/actions/workflows/rust.yml)
![Crates.io Total Downloads](https://img.shields.io/crates/d/rust-l402)
[![Documentation](https://img.shields.io/static/v1?logo=read-the-docs&label=docs.rs&message=rust_l402&color=informational)](https://docs.rs/rust_l402/latest/rust_l402/)

Rust L402 client module to consume L402 endpoints by using the [L402 Protocol](https://docs.lightning.engineering/the-lightning-network/l402)

⚠️**While this code hapanned many tests, it is still beta-quality experimental software. Use at your own risk. Independent audit is welcome.**

## About L402 Client

`rust_l402` is a comprehensive Rust crate designed to simplify the integration and handling of L402 protocol payments within the Lightning Network ecosystem. This SDK offers convenient abstractions for wallet interactions, invoice payments, and token management, making it an essential tool for developers working on Rust-based applications requiring L402 API access.

[Learn more about L402](https://l402.org) and [L402 Protocol by Lightning Labs](https://docs.lightning.engineering/the-lightning-network/l402)

## Features

- **L402 Client**: Composable L402 HTTP client to handle L402 API requests using Rust's `reqwest` library.
- **Wallet Interface**: Facilitates invoice payments through various wallet implementations, starting with `LND`support.
- **Token Store Interface**: Manages and stores L402 tokens, allowing for efficient retrieval based on URL, host, and path with support for closest match searching.
- **Proxy**: Provides a proxy service to handle L402 API requests, allowing for seamless integration with various L402-compliant payment gateways.
- **Nostr**: Integrates with the Nostr protocol for secure and decentralized communication and `NWC` wallet support.
- **NWC Wallet**: Supports the NWC (Nostr Wallet Connect) protocol for secure and decentralized communication and wallet management.
- **Paywall**: Enables the creation of paywalls for content access, supporting both L402 and Nostr-based paywalls.

## Getting Started

### Prerequisites

- Rust version 1.80 or higher
- Access to an L402 compliant payment gateway (API)
- Nostr wallet (optional)

### Installation

To start using the Rust L402 SDK, add it to your `Cargo.toml`:

```toml
[dependencies]
rust_l402 = "2.0.6-beta"
```

## Example Usage

This example demonstrates how to use the L402 client with the LND wallet to make a request to the `rnd.ln.sulu.sh/randomnumber` [API](https://docs.sulu.sh/docs/l402-demonstration) or via [FewSats](https://www.fewsats.com/), which returns a random number.

### Quick Start

```rust
use rust_l402::L402Client;

fn main() {
    // Initialize the wallet and client
    let client = L402Client::new(wallet);

    // Make a request to the API
    let response = client.get("https://rnd.ln.sulu.sh/randomnumber").send().unwrap();
    
    // Print the response
    println!("Response: {}", response.text().unwrap());
}
```

### Detailed Usage
```rust
use rust_l402::{L402Client, L402Wallet};
fn main() {
    // Initialize the wallet and client
    let wallet = L402Wallet::new(wallet_type);
    let client = L402Client::new(wallet);
    // Make a request to the API
    let response = client.get("https://rnd.ln.sulu.sh/randomnumber").send().unwrap();
    // Print the response
    println!("Response: {}", response.text().unwrap());
}
```

**This example provides a quick overview of how to utilize the `rust_l402` crate for making payments and handling L402 API requests in a Rust-based application.**
