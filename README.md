# Rust l402 Protocol🦀⚡

[![Bitcoin-only](https://img.shields.io/badge/bitcoin-only-FF9900?logo=bitcoin)](https://twentyone.world)
[![LN](https://img.shields.io/badge/lightning-792EE5?logo=lightning)](https://mempool.space/lightning)

Rust L402 client module to consume L402 endpoints. 

⚠️**This client is under development**

⚠️**Beta version**

# About L402 Library

`rust_l402` is a comprehensive Rust crate designed to simplify the integration and handling of L402 protocol payments within the Lightning Network ecosystem. This SDK offers convenient abstractions for wallet interactions, invoice payments, and token management, making it an essential tool for developers working on Rust-based applications requiring L402 API access.

## Features

- **L402 Client**: Composable L402 HTTP client to handle L402 API requests using Rust's `reqwest` library.
- **Wallet Interface**: Facilitates invoice payments through various wallet implementations, starting with `LND` wallet support.
- **Token Store Interface**: Manages and stores L402 tokens, allowing for efficient retrieval based on URL, host, and path with support for closest match searching.

## Getting Started

### Prerequisites

- Rust version 1.70 or higher
- Access to an L402 compliant payment gateway

### Installation

To start using the Rust L402 SDK, add it to your `Cargo.toml`:

```toml
[dependencies]
rust_l402 = "0.1.0-beta"
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

This example provides a quick overview of how to utilize the `rust_l402` crate for making payments and handling L402 API requests in a Rust-based application.

## Roadmap

- [x] Client
- [x] Tokenstore
- [x] Wallet (LND)
- [ ] Publish Crates
- [x] Cargo tests
- [x] Fix lib.rs
- [ ] FOSS 
- [ ] Alby wallet
