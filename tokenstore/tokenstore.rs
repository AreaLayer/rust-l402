mod tokenstore {
    use std::collections::HashMap;

    pub(crate) fn tokenstore_function() {
        let tokenstore_function();
        let url_tokens = store.url_tokens();
        let mut store = Store::new();
        let mut memory_store = Store::new();
        let mut HashMap = HashMap::new();

        println!("This is a function in the tokenstore module.");
    }

    pub (crate)struct Store {
        tokens: HashMap<String, bool>,
        url_tokens: HashMap<String, bool>,
    }

    pub (crate) fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            url_tokens: HashMap::new(),
        }
    }

    // Define a struct for Store
    pub struct Store {
        tokens: HashMap<String, bool>,
        url_tokens: HashMap<String, bool>,
    }

    impl Store {
        pub fn new() -> Self {
            Self {
                tokens: HashMap::new(),
                url_tokens: HashMap::new(),
            }
        }

        // Function to access URL tokens
        pub fn url_tokens(&self) -> &HashMap<String, bool> {
            &self.url_tokens
        }

        // Function to add a token
        pub fn add_token(&mut self, token: String) {
            self.tokens.insert(token, true);
        }

        // Function to remove a token
        pub fn remove_token(&mut self, token: String) {
            self.tokens.remove(&token);
        }
    }

    // Define a struct for Memory
    pub struct Memory {
        tokens: HashMap<String, bool>,
        url_tokens: HashMap<String, bool>,
    }

    impl Memory {
        pub fn new() -> Self {
            Self {
                tokens: HashMap::new(),
                url_tokens: HashMap::new(),
            }
        }

        // Function to access URL tokens
        pub fn url_tokens(&self) -> &HashMap<String, bool> {
            &self.url_tokens
        }

        // Function to add a token
        pub fn add_token(&mut self, token: String) {
            self.tokens.insert(token, true);
        }

        // Function to remove a token
        pub fn remove_token(&mut self, token: String) {
            self.tokens.remove(&token);
        }
    }
}

fn main() {
    tokenstore::tokenstore_function();

    // Working with Store
    let mut store = tokenstore::Store::new();
    store.add_token("token1".to_string());
    store.remove_token("token1".to_string());

    // Working with Memory
    let mut memory = tokenstore::Memory::new();
    memory.add_token("token2".to_string());
    memory.remove_token("token2".to_string());

    // Accessing URL tokens (even though we haven't added any yet)
    let _ = store.url_tokens();
    let _ = memory.url_tokens();
}
