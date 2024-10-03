use crate::Proxy;

pub (crate) struct ProxyFunction;

fn _l402_proxy_function() {
    println!("l402_proxy_function");
}
impl ProxyFunction {
    // Define a proxy module with a function
    pub fn proxy_function() {
        _l402_proxy_function();
        _proxy_function();
        _header_key_function();
        _invoice_function();
        _macaroon_function();
        _wallet_function();
        _store_function();
        _macaroon_store_function();
        println!("proxy_function");
        println!("Proxy function called.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Proxy;
    use crate::ProxyFunction;

    fn test_proxy_function() {
        ProxyFunction::proxy_function();
    }
}