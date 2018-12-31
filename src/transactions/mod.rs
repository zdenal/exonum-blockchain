transactions! {
    // Transaction group.
    pub CurrencyTransactions {
        const SERVICE_ID = SERVICE_ID;

        // Transaction type for creating a new wallet.
        struct TxCreateWallet {
            pub_key: &PublicKey,
            name: &str,
        }

        // Transaction type for transferring tokens between two wallets.
        struct TxTransfer {
            from: &PublicKey,
            to: &PublicKey,
            amount: u64,
            seed: u64,
        }
    }
}
