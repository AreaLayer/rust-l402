pub(crate) fn tokenstore_function() {
    println!("This is a function in the tokenstore module.");
}

impl store::Store {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            tokens: URLHashMap::new(),
        }
    }
    pub fn URLHashMap(&self) -> &URLHashMap {
        &self.URLHashMap
    }
    pub fn add_token(&mut self, token: String) {
        self.tokens.insert(token, true);
    }
    pub fn remove_token(&mut self, token: String) {
        self.tokens.remove(&token);
    }
}

impl memory {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            tokens: URLHashMap::new(),
        }
    }
    pub fn URLHashMap(&self) -> &URLHashMap {
        &self.URLHashMap
    }
    pub fn add_token(&mut self, token: String) {
        self.tokens.insert(token, true);
    }
    pub fn remove_token(&mut self, token: String) {
        self.tokens.remove(&token);
    }
}
