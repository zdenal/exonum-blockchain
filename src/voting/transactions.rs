use exonum::{
    blockchain::{ExecutionResult, Transaction},
    crypto::PublicKey,
    messages::Message,
    storage::Fork,
};

use voting::{
    errors::Error,
    schema::{VotingSchema, Candidate, Vote},
};

// Service identifier
const SERVICE_ID: u16 = 2;
const INIT_VOTES: u16 = 0;

transactions! {
    pub VotingTransactions {
        const SERVICE_ID = SERVICE_ID;

        struct VotingCreateCandidate {
            pub_key: &PublicKey,
            name: &str,
        }

        struct VotingVote {
            pub_key: &PublicKey,
            candidate: &PublicKey,
        }
    }
}

impl Transaction for VotingCreateCandidate {
    fn verify(&self) -> bool {
        println!("veryfying: {:?}", self);
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = VotingSchema::new(view);
        if schema.candidate(self.pub_key()).is_none() {
            let candidate = Candidate::new(self.pub_key(), self.name(), INIT_VOTES);
            println!("Create the candidate: {:?}", candidate);
            schema.candidates_mut().put(self.pub_key(), candidate);
            Ok(())
        } else {
            Err(Error::CandidateAlreadyExists)?
        }
    }
}

impl Transaction for VotingVote {
    fn verify(&self) -> bool {
        // Can't vote for yourself. Just playing with logic ...
        !self.verify_signature(self.candidate()) && self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = VotingSchema::new(view);

        if schema.candidate(self.candidate()).is_none() {
            Err(Error::CandidateNotFound)?
        } else if schema.vote(self.pub_key()).is_none() {
            let vote = Vote::new(self.pub_key(), self.candidate());
            println!("Create the vote: {:?}", vote);
            schema.votes_mut().put(self.pub_key(), vote);
            Ok(())
        } else {
            Err(Error::VoteAlreadyExists)?
        }
    }
}
