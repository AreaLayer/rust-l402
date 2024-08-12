mod client;
mod wallet;
mod tokenstore;

pub fn public_function() {
    client::client_function();
    wallet::wallet_function();
    tokenstore::tokenstore_function();
}

fn main() {
    public_function();
}
