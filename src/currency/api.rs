use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    crypto::PublicKey,
};

use currency::schema::{CurrencySchema, Wallet};

#[derive(Debug, Clone)]
pub struct CryptocurrencyApi;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WalletQuery {
    pub pub_key: PublicKey,
}

impl CryptocurrencyApi {
    pub fn get_wallet(state: &ServiceApiState, query: WalletQuery) -> api::Result<Wallet> {
        let snapshot = state.snapshot();
        let schema = CurrencySchema::new(snapshot);
        schema
            .wallet(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("\"Wallet not found\"".to_owned()))
    }

    pub fn get_wallets(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Wallet>> {
        let snapshot = state.snapshot();
        let schema = CurrencySchema::new(snapshot);
        let idx = schema.wallets();
        let wallets = idx.values().collect();
        Ok(wallets)
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/wallet", Self::get_wallet)
            .endpoint("v1/wallets", Self::get_wallets);
    }
}
