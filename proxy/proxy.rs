use tonic::transport::Server;
use tonic::{Request, Response, Status};
use rust_l042::Tokenstore;
use std::sync::Arc;
use tokio::sync::Mutex;

// Pub crate to Proxy
pub fn new(token_store: Arc<Mutex<Tokenstore>>, http_client: reqwest::Client, lnd_url: String, lnd_macaroon: String) -> Self {
    Proxy {
        token_store,
        http_client,
        lnd_url,
        lnd_macaroon,
    }
}// Pub struct to Proxy
pub struct Proxy {
    token_store: Arc<Mutex<Tokenstore>>,
    http_client: reqwest::Client,
    lnd_url: String,
    lnd_macaroon: String,
}
// Define your gRPC service trait here
pub mod l402_proxy {
    tonic::include_proto!("l402_proxy");
}

use l402_proxy::l402_proxy_server::{L402ProxyServer, L402ProxyServer};
use l402_proxy::{L402ProxyRequest, L402ProxyResponse};

#[derive(Debug, Default)]
pub struct L404ProxyImpl {
    // Optionally add state here
    token_store: Arc<Mutex<Tokenstore>>,
    http_client: reqwest::Client,
    lnd_url: String,
    lnd_macaroon: String,
}

#[tonic::async_trait]
impl L402Proxy for L402ProxyImpl {
    async fn my_method(
        &self,
        request: Request<L402ProxyRequest>,
    ) -> Result<Response<L402ProxyResponse>, Status> {
        // Handle the incoming request and proxy to LND
        // You would use a gRPC client here to forward the request to the LND gRPC server
        LND_PROXY_REQUEST.with_label_values(&["my_method"]).inc();
        let mut token_store = self.token_store.lock().await;
        
        // For demonstration purposes, return a dummy response
        let response = L402ProxyResponse {
            message: "Hello from proxy".into(),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let my_service = L402ProxyImpl::default();

    println!("Starting gRPC server at {:?}", addr);

    Server::builder()
        .add_service(L402ProxyServer::new(my_service))
        .serve(addr)
        .await?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use tonic::transport::Channel;
    use tonic::Request;
    use proxy_proto::proxy_server::Proxy;

    fn test_my_method() {
        let mut client = ProxyClient::connect("http://[::1]:50051").await.unwrap();
        let request = Request::new(L402ProxyRequest {
            message: "Hello from client".into(),
        });
        let response = client.my_method(request).await.unwrap();
        assert_eq!(response.into_inner().message, "Hello from proxy");
    }
}