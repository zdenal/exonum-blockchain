use voting::proto;
use super::SERVICE_ID;
use exonum::{
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxCreateCandidate")]
pub struct TxCreateCandidate {
    pub pub_key: PublicKey,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxVote")]
pub struct TxVote {
    pub voter: PublicKey,
    pub candidate: PublicKey,
    /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
    pub seed: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum VotingTransactions {
    CreateCandidate(TxCreateCandidate),
    Vote(TxVote),
}

impl TxCreateCandidate {
    #[doc(hidden)]
    pub fn sign(name: &str, pk: &PublicKey, sk: &SecretKey) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                pub_key: *pk,
                name: name.to_owned(),
            },
            SERVICE_ID,
            *pk,
            sk,
            )
    }
}

impl TxVote {
    #[doc(hidden)]
    pub fn sign(
        voter: &PublicKey,
        candidate: &PublicKey,
        seed: u64,
        pk: &PublicKey,
        sk: &SecretKey,
        ) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                voter: *voter,
                candidate: *candidate,
                seed,
            },
            SERVICE_ID,
            *pk,
            sk,
            )
    }
}
