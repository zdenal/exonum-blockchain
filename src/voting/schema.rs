use exonum::{
    storage::{Fork, MapIndex, Snapshot},
};

use voting::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Candidate")]
pub struct Candidate {
    pub pub_key: String,
    pub name: String,
    pub votes: u64,
}

impl Candidate {
    pub fn new(pub_key: &str, name: &str, votes: u64) -> Self {
        Self {
            pub_key: pub_key.to_owned(),
            name: name.to_owned(),
            votes,
        }
    }

    pub fn increase_votes(self) -> Self {
        let votes = self.votes + 1;
        Self::new(&self.pub_key, &self.name, votes)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Vote")]
pub struct Vote {
    pub voter: String,
    pub candidate: String,
}
impl Vote {
    pub fn new(voter: &str, candidate: &str) -> Self {
        Self {
            voter: voter.to_owned(),
            candidate: candidate.to_owned()
        }
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

    pub fn candidates(&self) -> MapIndex<&dyn Snapshot, String, Candidate> {
        MapIndex::new("cryptocurrency.candidates", self.view.as_ref())
    }

    pub fn candidate(&self, pub_key: &str) -> Option<Candidate> {
        self.candidates().get(pub_key)
    }

    pub fn votes(&self) -> MapIndex<&dyn Snapshot, String, Vote> {
        MapIndex::new("cryptocurrency.votes", self.view.as_ref())
    }

    pub fn vote(&self, pub_key: &str) -> Option<Vote> {
        self.votes().get(pub_key)
    }
}

impl<'a> VotingSchema<&'a mut Fork> {
    pub fn candidates_mut(&mut self) -> MapIndex<&mut Fork, String, Candidate> {
        MapIndex::new("cryptocurrency.candidates", &mut self.view)
    }
    pub fn votes_mut(&mut self) -> MapIndex<&mut Fork, String, Vote> {
        MapIndex::new("cryptocurrency.votes", &mut self.view)
    }
}

