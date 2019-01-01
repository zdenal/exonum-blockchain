#[macro_use]
extern crate exonum;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use exonum::{
    api::{ServiceApiState, ServiceApiBuilder, self},
    blockchain::{Service, Transaction, TransactionSet},
    crypto::{Hash, PublicKey},
    encoding,
    messages::{RawTransaction},
    node::{TransactionSend},
    storage::{Snapshot},
};

mod currency;
use currency::{
    schema::{Wallet, CurrencySchema},
    transactions::CurrencyTransactions
};

// Service identifier
const SERVICE_ID: u16 = 1;

struct CryptocurrencyApi;

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    // Hash of the transaction.
    pub tx_hash: Hash,
}

impl CryptocurrencyApi {
    fn post_transaction(state: &ServiceApiState, query: CurrencyTransactions)
     -> api::Result<TransactionResponse> {
        let transaction: Box<Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }
}

#[derive(Deserialize)]
/// The structure describes the query parameters for the `get_wallet` endpoint.
struct WalletQuery {
    /// Public key of the queried wallet.
    pub_key: PublicKey,
}

impl CryptocurrencyApi {
    /// Endpoint for getting a single wallet.
    fn get_wallet(state: &ServiceApiState, query: WalletQuery)
     -> api::Result<Wallet> {
        let snapshot = state.snapshot();
        let schema = CurrencySchema::new(snapshot);
        schema
            .wallet(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Wallet not found".to_owned()))
    }

    /// Endpoint for dumping all wallets from the storage.
    fn get_wallets(state: &ServiceApiState, _query: ())
     -> api::Result<Vec<Wallet>> {
        let snapshot = state.snapshot();
        let schema = CurrencySchema::new(snapshot);
        let idx = schema.wallets();
        let wallets = idx.values().collect();
        Ok(wallets)
    }
}

impl CryptocurrencyApi {
    fn wire(builder: &mut ServiceApiBuilder) {
        // Binds handlers to specific routes.
        builder
            .public_scope()
            // Read only endpoints uses `GET` method.
            .endpoint("v1/wallet", Self::get_wallet)
            .endpoint("v1/wallets", Self::get_wallets)
            // But for methods that can modify service state you should use
            // `endpoint_mut` that uses `POST` method.
            .endpoint_mut("v1/wallets", Self::post_transaction)
            .endpoint_mut("v1/wallets/transfer", Self::post_transaction);
    }
}

pub struct CurrencyService;

impl Service for CurrencyService {
    fn service_name(&self) -> &'static str { "cryptocurrency" }

    fn service_id(&self) -> u16 { SERVICE_ID }

    fn tx_from_raw(&self, raw: RawTransaction) ->
        Result<Box<Transaction>, encoding::Error>
    {
        let tx = CurrencyTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        CryptocurrencyApi::wire(builder)
    }
}
