mod wallet {
    use lnd_grpc::lnrpc::{wallet_unlocker_client::WalletUnlockerClient, UnlockWalletRequest};
    use alby_greenlight::alby_greenlight_client::AlbyGreenlightClient;
    use sha2::{Sha256, Digest};
    use std::error::Error;

    // PreImage struct
    pub struct PreImage {
        pub preimage: Vec<u8>,
        pub hash: Vec<u8>,
        pub hash_type: u8,
        pub timestamp: u64,
        pub preimage_hash: Vec<u8>,
    }

    // PreImageHash struct
    pub struct PreImageHash {
        pub preimage_hash: Vec<u8>,
    }

    // Wallet module functions
    pub(crate) async fn wallet_function() -> Result<(), Box<dyn Error>> {
        // Example Alby Greenlight and LND client initialization
        let mut greenlight_client = AlbyGreenlightClient::new("http://localhost:8080").await?;
        let mut lnd_client = WalletUnlockerClient::connect("http://localhost:10009").await?;

        println!("This is a function in the wallet module.");

        // Create a new PreImage
        let preimage_data = vec![1, 2, 3, 4];  // Example preimage data
        let preimage = PreImage::new(preimage_data);

        println!("Created a new PreImage with data: {:?}", preimage.preimage);

        Ok(())
    }

    // Unlock wallet function
    pub async fn unlock_wallet(client: &mut WalletUnlockerClient, wallet_password: &str) -> Result<(), Box<dyn Error>> {
        let request = UnlockWalletRequest {
            wallet_password: wallet_password.to_string(),
        };
        let _response = client.unlock_wallet(request).await?;
        Ok(())
    }

    // PreImage struct implementation
    impl PreImage {
        // Function to create a new PreImage
        pub fn new(preimage: Vec<u8>) -> Self {
            let mut hasher = Sha256::new();
            hasher.update(&preimage);
            let hash = hasher.finalize().to_vec();

            Self {
                preimage: preimage.clone(),
                hash: hash.clone(),
                hash_type: 0,  // Example: SHA-256
                timestamp: Self::current_timestamp(),
                preimage_hash: hash,
            }
        }

        // Function to get the current timestamp
        pub fn current_timestamp() -> u64 {
            use std::time::{SystemTime, UNIX_EPOCH};
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
            since_the_epoch.as_secs()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use lnd_grpc::lnrpc::wallet_unlocker_client::WalletUnlockerClient;
        use tokio;

        #[tokio::test]
        async fn test_unlock_wallet() -> Result<(), Box<dyn Error>> {
            // Test unlocking the wallet with a mock client
            let mut client = WalletUnlockerClient::connect("http://localhost:10009").await?;
            let result = unlock_wallet(&mut client, "testpassword").await;
            assert!(result.is_ok());
            Ok(())
        }

        #[test]
        fn test_preimage_creation() {
            let preimage_data = vec![1, 2, 3, 4];
            let preimage = PreImage::new(preimage_data);
            assert_eq!(preimage.preimage.len(), 4);
            assert_eq!(preimage.hash.len(), 32);
        }
    }
}

fn main() {
    // Example usage
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(wallet::wallet_function());
}
