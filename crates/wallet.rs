/// Represents a collection of wallets.
pub(crate) struct Wallets {
    /// A vector to store individual `Wallet` instances.
    wallets: Vec<Wallet>,
}

impl Wallets {
    /// Creates a new `Wallets` instance with an empty vector.
    ///
    /// # Returns
    /// 
    /// * `Wallets` - An instance of the `Wallets` struct with no wallets in it.
    pub fn new() -> Self {
        Wallets {
            wallets: Vec::new(),
        }
    }
}

impl Wallets {
    /// Adds a wallet to the collection of wallets.
    ///
    /// # Parameters
    ///
    /// * `wallet` - The `Wallet` instance to be added to the collection.
    ///
    /// # Example
    ///
    /// ```
    /// let mut wallets = Wallets::new();
    /// let wallet = Wallet::new();
    /// wallets.add_wallet(wallet);
    /// ```
    pub fn add_wallet(&mut self, wallet: Wallet) {
        self.wallets.push(wallet);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test the `add_wallet` method to ensure a wallet is added correctly.
    #[test]
    fn test_add_wallet() {
        let mut wallets = Wallets::new();
        let wallet = Wallet::new();
        wallets.add_wallet(wallet);

        // Assert that the collection now contains one wallet.
        assert!(
            wallets.wallets.len() == 1,
            "Wallets should have 1 wallet"
        );
    }
}
