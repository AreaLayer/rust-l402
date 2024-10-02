use crate::tokenstore;
use l402::*;

pub(crate) struct TokenStore;

impl TokenStore {
    // Define a token store module with a function
    pub fn token_store() {
        _l402_token_store();
        _token_store();
        _token_store_function();
        println!("token_store");
        println!("Token store function called.");
    }
}