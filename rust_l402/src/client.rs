mod client;

pub fn client() {
    crate::client::client();
}

pub fn server() {
    crate::server::server();
}

pub fn server_with_client() {
    crate::server::server_with_client();
}
