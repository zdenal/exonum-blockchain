use currency::proto;
use super::SERVICE_ID;
use exonum::{
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxCreateWallet")]
pub struct TxCreateWallet {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxTransfer")]
pub struct TxTransfer {
    pub to: PublicKey,
    pub amount: u64,
    /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
    pub seed: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum CurrencyTransactions {
    CreateWallet(TxCreateWallet),
    Transfer(TxTransfer),
}

impl TxCreateWallet {
    #[doc(hidden)]
    pub fn sign(name: &str, pk: &PublicKey, sk: &SecretKey) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                name: name.to_owned(),
            },
            SERVICE_ID,
            *pk,
            sk,
            )
    }
}

impl TxTransfer {
    #[doc(hidden)]
    pub fn sign(
        to: &PublicKey,
        amount: u64,
        seed: u64,
        pk: &PublicKey,
        sk: &SecretKey,
        ) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                to: *to,
                amount,
                seed,
            },
            SERVICE_ID,
            *pk,
            sk,
            )
    }
}
