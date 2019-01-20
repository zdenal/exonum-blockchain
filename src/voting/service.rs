use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{Service, Transaction, TransactionSet},
    crypto::{Hash, PublicKey},
    encoding,
    messages::RawTransaction,
    node::TransactionSend,
    storage::Snapshot,
};

use voting::{
    schema::{VotingSchema, Candidate, Vote},
    transactions::VotingTransactions,
};

// Service identifier
const SERVICE_ID: u16 = 2;

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    // Hash of the transaction.
    pub tx_hash: Hash,
}

struct VotingApi;

// POST
impl VotingApi {
    fn post_transaction(
        state: &ServiceApiState,
        query: VotingTransactions,
    ) -> api::Result<TransactionResponse> {
        let transaction: Box<Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }
}

// GET
#[derive(Deserialize)]
struct VoteQuery {
    /// Public key of the queried vote.
    pub_key: PublicKey,
}

#[derive(Deserialize)]
struct CandidateQuery {
    /// Public key of the queried candidate.
    pub_key: PublicKey,
}

impl VotingApi {
    fn get_vote(state: &ServiceApiState, query: VoteQuery) -> api::Result<Vote> {
        let snapshot = state.snapshot();
        let schema = VotingSchema::new(snapshot);
        schema
            .vote(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Vote not found".to_owned()))
    }

    fn get_votes(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Vote>> {
        let snapshot = state.snapshot();
        let schema = VotingSchema::new(snapshot);
        let idx = schema.votes();
        let votes = idx.values().collect();
        Ok(votes)
    }

    fn get_candidate(state: &ServiceApiState, query: CandidateQuery) -> api::Result<Candidate> {
        let snapshot = state.snapshot();
        let schema = VotingSchema::new(snapshot);
        schema
            .candidate(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Candidate not found".to_owned()))
    }

    fn get_candidates(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Candidate>> {
        let snapshot = state.snapshot();
        let schema = VotingSchema::new(snapshot);
        let idx = schema.candidates();
        let candidates = idx.values().collect();
        Ok(candidates)
    }
}

// Wire endpoints
impl VotingApi {
    fn wire(builder: &mut ServiceApiBuilder) {
        // Binds handlers to specific routes.
        builder
            .public_scope()
            // GET
            .endpoint("v1/vote", Self::get_vote)
            .endpoint("v1/votes", Self::get_votes)
            .endpoint("v1/candidate", Self::get_candidate)
            .endpoint("v1/candidates", Self::get_candidates)
            // POST
            .endpoint_mut("v1/votes", Self::post_transaction)
            .endpoint_mut("v1/candidates", Self::post_transaction);
    }
}

pub struct VotingService;

impl Service for VotingService {
    fn service_name(&self) -> &'static str {
        "voting"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx = VotingTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        VotingApi::wire(builder)
    }
}
