#![allow(bare_trait_objects)]
#![allow(renamed_and_removed_lints)]

pub use self::voting::{TxCreateCandidate, TxVote, Candidate, Vote};

include!(concat!(env!("OUT_DIR"), "/voting_protobuf_mod.rs"));

//use exonum::proto::schema::*;
