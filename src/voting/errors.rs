#![allow(bare_trait_objects)]

use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Candidate already exists")]
    CandidateAlreadyExists = 0,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}
