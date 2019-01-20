use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Candidate already exists")]
    CandidateAlreadyExists = 0,

    #[fail(display = "Vote already exists")]
    VoteAlreadyExists = 1,

    #[fail(display = "Candidate doesn't exist")]
    CandidateNotFound = 2,
}

// Conversion between service-specific errors and the standard error type
// that can be emitted by transactions.
impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}
