use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    crypto::PublicKey,
};

use voting::schema::{VotingSchema, Candidate};

#[derive(Debug, Clone)]
pub struct VotingApi;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandidateQuery {
    pub pub_key: String,
}

impl VotingApi {
    pub fn get_candidate(state: &ServiceApiState, query: CandidateQuery) -> api::Result<Candidate> {
        let snapshot = state.snapshot();
        let schema = VotingSchema::new(snapshot);
        schema
            .candidate(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("\"Candidate not found\"".to_owned()))
    }

    pub fn get_candidates(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Candidate>> {
        let snapshot = state.snapshot();
        let schema = VotingSchema::new(snapshot);
        let idx = schema.candidates();
        let candidates = idx.values().collect();
        Ok(candidates)
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/candidate", Self::get_candidate)
            .endpoint("v1/candidates", Self::get_candidates);
    }
}
