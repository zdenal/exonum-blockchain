use exonum::{
    blockchain::{ExecutionResult, Transaction},
    crypto::PublicKey,
    messages::Message,
    storage::Fork,
};

use currency::{
    errors::Error,
    schema::{CurrencySchema, Wallet},
};

// Service identifier
const SERVICE_ID: u16 = 1;
// Starting balance of a newly created wallet
const INIT_BALANCE: u64 = 100;

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

impl Transaction for TxCreateWallet {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(view);
        if schema.wallet(self.pub_key()).is_none() {
            let wallet = Wallet::new(self.pub_key(), self.name(), INIT_BALANCE);
            println!("Create the wallet: {:?}", wallet);
            schema.wallets_mut().put(self.pub_key(), wallet);
            Ok(())
        } else {
            Err(Error::WalletAlreadyExists)?
        }
    }
}

impl Transaction for TxTransfer {
    fn verify(&self) -> bool {
        (*self.from() != *self.to()) && self.verify_signature(self.from())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(view);

        let sender = match schema.wallet(self.from()) {
            Some(val) => val,
            None => Err(Error::SenderNotFound)?,
        };

        let receiver = match schema.wallet(self.to()) {
            Some(val) => val,
            None => Err(Error::ReceiverNotFound)?,
        };

        let amount = self.amount();
        if sender.balance() >= amount {
            let sender = sender.decrease(amount);
            let receiver = receiver.increase(amount);
            println!("Transfer between wallets: {:?} => {:?}", sender, receiver);
            let mut wallets = schema.wallets_mut();
            wallets.put(self.from(), sender);
            wallets.put(self.to(), receiver);
            Ok(())
        } else {
            Err(Error::InsufficientCurrencyAmount)?
        }
    }
}
