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
