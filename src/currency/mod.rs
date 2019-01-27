pub mod schema;
pub mod transactions;
pub mod contracts;
pub mod errors;
pub mod api;
pub mod proto;

use exonum::{
    api::ServiceApiBuilder,
    blockchain::{Service, Transaction, TransactionSet},
    crypto::Hash,
    messages::RawTransaction,
    storage::Snapshot,
};

use currency::{api::CryptocurrencyApi, transactions::CurrencyTransactions};

pub const SERVICE_ID: u16 = 1;

#[derive(Debug)]
pub struct CurrencyService;

impl Service for CurrencyService {
    fn service_name(&self) -> &'static str {
        "cryptocurrency"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
        let tx = CurrencyTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        CryptocurrencyApi::wire(builder);
    }
}
