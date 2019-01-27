#![allow(bare_trait_objects)]

use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Candidate already exists")]
    CandidateAlreadyExists = 0,

    #[fail(display = "Create Candidate denied")]
    CreateCandidateDenied = 1,

    #[fail(display = "Candidate doesn't exist")]
    CandidateNotFound = 2,

    #[fail(display = "Sender same as receiver")]
    SenderSameAsReceiver = 3,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}
