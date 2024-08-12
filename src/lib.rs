mod client;
mod wallet;
mod tokenstore;

pub fn public_function() {
    crate::my_module::internal_function();
}

// src/my_module.rs
pub(crate) fn internal_function() {
    println!("This is an internal function.");
}
