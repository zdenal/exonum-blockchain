use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{Schema, Block},
    helpers::Height,
    crypto::Hash
};

#[derive(Debug, Clone)]
pub struct ChainApi;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeightQuery {
    pub height: u64,
}

impl ChainApi {
    pub fn get_blocks(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Block>> {
        let schema = Schema::new(state.snapshot());
        let idx = schema.blocks();
        let blocks = idx.values().collect();
        Ok(blocks)
    }

    pub fn get_block_transactions(state: &ServiceApiState, query: HeightQuery) -> api::Result<Vec<Hash>> {
        let schema = Schema::new(state.snapshot());
        let txs = schema.block_transactions(Height(query.height));
        let transactions = txs.into_iter().collect();
        println!("transactions: {:?}", transactions);
        Ok(transactions)
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/blocks", Self::get_blocks)
            .endpoint("v1/block_transactions", Self::get_block_transactions);
    }
}

