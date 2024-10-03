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

fn _l402_token_store() {
    println!("l402_token_store");
    println!("Token store function called.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenstore;
    use tokenstore::*;
    use l402::*;

    fn test_token_store() {
        let token_store = TokenStore::token_store
        .token_store();
    assert_eq!(token_store, "token_store");
    }
    }