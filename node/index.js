const Exonum = require('exonum-client');
const axios = require('axios');
const cTable = require('console.table');
const $protobuf = require('protobufjs/light');
const Root = $protobuf.Root;
const Type = $protobuf.Type;
const Field = $protobuf.Field;

CURRENCY_SERVICE_ID = 1;
VOTING_SERVICE_ID = 2;
TRANSACTIONS_URL = 'http://127.0.0.1:8000/api/explorer/v1/transactions';

// For generate new one use
const candidateKeyPair = Exonum.keyPair();
const voterKeyPair = Exonum.keyPair();
authorityKeyPair = {
  publicKey: 'c97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
  secretKey:
    'a4033ba9c9ee6098f32a6f904f19a7feea155c36c5116d578a8399f3d2eea67dc97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
};

const CreateSchema = new Type('CreateSchema').add(
  new Field('name', 1, 'string'),
);

const CreateWalletTx = Exonum.newTransaction({
  author: authorityKeyPair.publicKey,
  service_id: CURRENCY_SERVICE_ID,
  message_id: 0,
  schema: CreateSchema,
});

const CreateCandidateSchema = new Type('CreateCandidateSchema')
  .add(new Field('pub_key', 1, 'string'))
  .add(new Field('name', 2, 'string'));

const CreateCandidatetTx = Exonum.newTransaction({
  author: authorityKeyPair.publicKey,
  service_id: VOTING_SERVICE_ID,
  message_id: 0,
  schema: CreateCandidateSchema,
});

function createWallet(name, keyPair) {
  const data = {
    name: name,
  };

  //const signature = CreateWalletTx.sign(keyPair.secretKey, data);
  //console.log('signature: ', signature);

  CreateWalletTx.send(TRANSACTIONS_URL, data, keyPair.secretKey).catch(e =>
    console.log(e),
  );
}

function createCandidate(name, candidateKeyPair, authorityKeyPair) {
  const data = {
    pub_key: authorityKeyPair.publicKey,
    name: name,
  };

  console.log(data);
  //const signature = CreateCandidatetTx.sign(authorityKeyPair.secretKey, data);
  //console.log('signature: ', signature);

  CreateCandidatetTx.send(TRANSACTIONS_URL, data, authorityKeyPair.secretKey)
    .then(r => console.log(r))
    .catch(e => console.log(e));
}

async function getCandidates() {
  const response = await axios.get(
    'http://127.0.0.1:8000/api/services/voting/v1/candidates',
  );
  console.table(response.data);
}

async function getBlocks() {
  const response = await axios.get(
    'http://127.0.0.1:8000/api/services/chain/v1/blocks',
  );
  console.table(response.data.filter(block => block.tx_count > 0));
}

async function getBlockTransactions(height) {
  const response = await axios.get(
    'http://127.0.0.1:8000/api/services/chain/v1/block_transactions',
    {
      params: {
        height: height,
      },
    },
  );
  console.log(response.data);
}

//createWallet('John', authorityKeyPair);
//createCandidate('Alois', candidateKeyPair, authorityKeyPair);
//getCandidates();
//getBlocks();
getBlockTransactions(1904);
