use exonum::{
    crypto::PublicKey,
    storage::{Fork, MapIndex, Snapshot},
};

use voting::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Candidate")]
pub struct Candidate {
    pub pub_key: PublicKey,
    pub name: String,
    pub votes: u64,
}

    impl Candidate {
        pub fn new(&pub_key: &PublicKey, name: &str, votes: u64) -> Self {
            Self {
                pub_key,
                name: name.to_owned(),
                votes,
            }
        }

        pub fn increase_votes(self) -> Self {
            let votes = self.votes + 1;
            Self::new(&self.pub_key, &self.name, votes)
        }
    }

#[derive(Debug)]
pub struct VotingSchema<T> {
    view: T,
}

impl<T: AsRef<dyn Snapshot>> VotingSchema<T> {
    pub fn new(view: T) -> Self {
        VotingSchema { view }
    }

    pub fn candidates(&self) -> MapIndex<&dyn Snapshot, PublicKey, Candidate> {
        MapIndex::new("cryptocurrency.candidates", self.view.as_ref())
    }

    pub fn candidate(&self, pub_key: &PublicKey) -> Option<Candidate> {
        self.candidates().get(pub_key)
    }
}

impl<'a> VotingSchema<&'a mut Fork> {
    pub fn candidates_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Candidate> {
        MapIndex::new("cryptocurrency.candidates", &mut self.view)
    }
}
