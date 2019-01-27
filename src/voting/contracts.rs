use exonum::blockchain::{ExecutionResult, Transaction, TransactionContext};

use voting::{
    errors::Error,
    transactions::{TxCreateCandidate, TxVote},
    schema::{VotingSchema, Candidate, Vote}
};

const INIT_VOTES: u64 = 0;
const AUTHORITY: &str = "c97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550";

impl Transaction for TxCreateCandidate {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let author = context.author();
        let view = context.fork();
        let mut schema = VotingSchema::new(view);

        if author.to_hex() != AUTHORITY {
            println!("Create the candidate denied: {:?}", author);
            Err(Error::CreateCandidateDenied)?
        }

        if schema.candidate(&self.pub_key).is_none() {
            let candidate = Candidate::new(&self.pub_key, &self.name, INIT_VOTES);
            println!("Create the candidate: {:?}", candidate);
            schema.candidates_mut().put(&self.pub_key, candidate);
            Ok(())
        } else {
            Err(Error::CandidateAlreadyExists)?
        }
    }
}

impl Transaction for TxVote {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let author = context.author();
        let view = context.fork();

        if self.voter == self.candidate {
            Err(Error::SenderSameAsReceiver)?
        }

        let mut schema = VotingSchema::new(view);

        let candidate = match schema.candidate(&self.candidate) {
            Some(val) => val,
            None => Err(Error::CandidateNotFound)?,
        };

        let vote = Vote::new(&self.voter, &self.candidate);
        let candidate = candidate.increase_votes();
        println!("Voting: {:?} => {:?}", author, candidate);
        schema.candidates_mut().put(&author, candidate);
        schema.votes_mut().put(&self.voter, vote);

        Ok(())
    }
}
