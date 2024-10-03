use crate::Wallet;

pub (crate) struct Wallets {
    wallets: Vec<Wallet>
}
impl Wallets {
    pub fn new() -> Self {
        Wallets {
            wallets: Vec::new()
        }
    }
}
impl Wallets {
    pub fn add_wallet(&mut self, wallet: Wallet) {
        self.wallets.push(wallet);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_wallet() {
        let mut wallets = Wallets::new();
        let wallet = Wallet::new();
        wallets.add_wallet(wallet);
        assert
        (
                wallets.wallets.len() == 1,
                "Wallets should have 1 wallet"
            );
    }
}