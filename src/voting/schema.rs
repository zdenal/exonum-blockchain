use exonum::crypto::PublicKey;
use exonum::storage::{Fork, MapIndex, Snapshot};

encoding_struct! {
    struct Vote {
        pub_key: &PublicKey,
        candidate: &PublicKey
    }
}

encoding_struct! {
    struct Candidate {
        pub_key: &PublicKey,
        name: &str,
        votes: u16
    }
}

pub struct VotingSchema<T> {
    view: T,
}

impl<T: AsRef<Snapshot>> VotingSchema<T> {
    pub fn new(view: T) -> Self {
        VotingSchema { view }
    }

    pub fn votes(&self) -> MapIndex<&Snapshot, PublicKey, Vote> {
        MapIndex::new("voting.votes", self.view.as_ref())
    }

    pub fn vote(&self, pub_key: &PublicKey) -> Option<Vote> {
        self.votes().get(pub_key)
    }

    pub fn candidates(&self) -> MapIndex<&Snapshot, PublicKey, Candidate> {
        MapIndex::new("voting.candidates", self.view.as_ref())
    }

    pub fn candidate(&self, pub_key: &PublicKey) -> Option<Candidate> {
        self.candidates().get(pub_key)
    }
}

impl<'a> VotingSchema<&'a mut Fork> {
    pub fn votes_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Vote> {
        MapIndex::new("voting.votes", &mut self.view)
    }

    pub fn candidates_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Candidate> {
        MapIndex::new("voting.candidates", &mut self.view)
    }
}
