use exonum::blockchain::{ExecutionResult, Transaction, TransactionContext};

use voting::{
    errors::Error,
    transactions::{TxCreateCandidate, TxVote},
    schema::{VotingSchema, Candidate}
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

        if schema.candidate(&author).is_none() {
            let candidate = Candidate::new(&author, &self.name, INIT_VOTES);
            println!("Create the candidate: {:?}", candidate);
            schema.candidates_mut().put(&author, candidate);
            Ok(())
        } else {
            Err(Error::CandidateAlreadyExists)?
        }
    }
}

impl Transaction for TxVote {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        // IMPLEMENT
        Ok(())
    }
}
