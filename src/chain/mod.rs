pub mod api;

pub const SERVICE_ID: u16 = 3;

use exonum::{
    api::ServiceApiBuilder,
    blockchain::{Service, Transaction, TransactionSet},
    crypto::Hash,
    messages::RawTransaction,
    storage::Snapshot,
};

use chain::{api::ChainApi};
use voting::{transactions::VotingTransactions};

#[derive(Debug)]
pub struct ChainService;

impl Service for ChainService {
    fn service_name(&self) -> &'static str {
        "chain"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
        // Not using transactions ... only for ad-hoc Service traid satisfaction
        let tx = VotingTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        ChainApi::wire(builder);
    }
}
