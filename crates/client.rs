use crate::Client;
use l402::*;

pub(crate) struct ClientFunction;

impl ClientFunction {
    // Define a client module with a function
    pub fn client_function() {
        _l402_client_function();
        _client_function();
        _header_key_function();
        _invoice_function();
        _macaroon_function();
        _wallet_function();
        _store_function();
        _macaroon_store_function();
        println!("client_function");
        println!("Client function called.");
    }
}

fn _l402_client_function() {
    let client = Client::new();
    let result = client.l402_client_function();
    println!("l402_client_function result: {}", result);
}

fn _client_function() {
    let client = Client::new();
    let result = client.client_function();
    println!("client_function result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;
    use l402::*;
    use std::env;

   fn test_l402_client_function() {
        let client = Client::new();
        let result = client.l402_client_function();
        assert_eq!(result, "l402_client_function");
    }
fn test_client_function() {
        let client = Client::new();
        let result = client.client_function();
        assert_eq!(result, "client_function");
    }

    fn test_header_key_function() {
        let client = Client::new();
        let result = client.header_key_function();
        assert_eq!(result, "header_key_function");
    }
    fn test_invoice_function() {
        let client = Client::new();
        let result = client.invoice_function();
        assert_eq!(result, "invoice_function");
    }

    fn test_macaroon_function() {
        let client = Client::new();
        let result = client.macaroon_function();
        assert_eq!(result, "macaroon_function");
    }

}