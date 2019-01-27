use exonum::blockchain::{ExecutionResult, Transaction, TransactionContext};

use currency::{
    errors::Error,
    transactions::{TxCreateWallet, TxTransfer},
};
use currency::schema::{CurrencySchema, Wallet};

const INIT_BALANCE: u64 = 100;

impl Transaction for TxCreateWallet {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let author = context.author();
        let view = context.fork();
        let mut schema = CurrencySchema::new(view);
        if schema.wallet(&author).is_none() {
            let wallet = Wallet::new(&author, &self.name, INIT_BALANCE);
            println!("Create the wallet: {:?}", wallet);
            schema.wallets_mut().put(&author, wallet);
            Ok(())
        } else {
            Err(Error::WalletAlreadyExists)?
        }
    }
}

impl Transaction for TxTransfer {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let author = context.author();
        let view = context.fork();

        if author == self.to {
            Err(Error::SenderSameAsReceiver)?
        }

        let mut schema = CurrencySchema::new(view);

        let sender = match schema.wallet(&author) {
            Some(val) => val,
            None => Err(Error::SenderNotFound)?,
        };

        let receiver = match schema.wallet(&self.to) {
            Some(val) => val,
            None => Err(Error::ReceiverNotFound)?,
        };

        let amount = self.amount;
        if sender.balance >= amount {
            let sender = sender.decrease(amount);
            let receiver = receiver.increase(amount);
            println!("Transfer between wallets: {:?} => {:?}", sender, receiver);
            let mut wallets = schema.wallets_mut();
            wallets.put(&author, sender);
            wallets.put(&self.to, receiver);
            Ok(())
        } else {
            Err(Error::InsufficientCurrencyAmount)?
        }
    }
}
