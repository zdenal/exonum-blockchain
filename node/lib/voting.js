const Exonum = require('exonum-client');
const axios = require('axios');
const cTable = require('console.table');
const $protobuf = require('protobufjs/light');
const Root = $protobuf.Root;
const Type = $protobuf.Type;
const Field = $protobuf.Field;

VOTING_SERVICE_ID = 2;
TRANSACTIONS_URL = 'http://127.0.0.1:8000/api/explorer/v1/transactions';

authorityKeyPair = {
  publicKey: 'c97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
  secretKey:
    'a4033ba9c9ee6098f32a6f904f19a7feea155c36c5116d578a8399f3d2eea67dc97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
};

const CreateCandidateSchema = new Type('CreateCandidateSchema')
  .add(new Field('pub_key', 1, 'string'))
  .add(new Field('name', 2, 'string'));

const CreateCandidatetTx = Exonum.newTransaction({
  author: authorityKeyPair.publicKey,
  service_id: VOTING_SERVICE_ID,
  message_id: 0,
  schema: CreateCandidateSchema,
});

const VoteSchema = new Type('VoteSchema')
  .add(new Field('voter', 1, 'string'))
  .add(new Field('candidate', 2, 'string'));

function voteTx(author) {
  return Exonum.newTransaction({
    author: author.publicKey,
    service_id: VOTING_SERVICE_ID,
    message_id: 1,
    schema: VoteSchema,
  });
}

exports.createCandidate = function(name, candidateKeyPair, authorityKeyPair) {
  const data = {
    pub_key: candidateKeyPair.publicKey,
    name: name,
  };

  console.log(data);
  //const signature = CreateCandidatetTx.sign(authorityKeyPair.secretKey, data);
  //console.log('signature: ', signature);

  CreateCandidatetTx.send(TRANSACTIONS_URL, data, authorityKeyPair.secretKey)
    .then(r => console.log(r))
    .catch(e => console.log(e));
};

exports.vote = function(candidate, voter) {
  const data = {
    voter: voter.publicKey,
    candidate: candidate.publicKey,
  };

  console.log(data);

  VoteTx(voter)
    .send(TRANSACTIONS_URL, data, voter.secretKey)
    .then(r => console.log(r))
    .catch(e => console.log(e));
};

exports.getCandidates = async function() {
  const response = await axios.get(
    'http://127.0.0.1:8000/api/services/voting/v1/candidates',
  );
  console.table(response.data);
};
