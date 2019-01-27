pub mod schema;
pub mod transactions;
pub mod contracts;
pub mod errors;
pub mod api;
pub mod proto;

pub const SERVICE_ID: u16 = 2;

use exonum::{
    api::ServiceApiBuilder,
    blockchain::{Service, Transaction, TransactionSet},
    crypto::Hash,
    messages::RawTransaction,
    storage::Snapshot,
};

use voting::{api::VotingApi, transactions::VotingTransactions};

#[derive(Debug)]
pub struct VotingService;

impl Service for VotingService {
    fn service_name(&self) -> &'static str {
        "voting"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
        let tx = VotingTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        VotingApi::wire(builder);
    }
}
