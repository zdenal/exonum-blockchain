const Exonum = require('exonum-client');
const $protobuf = require('protobufjs/light');
const Root = $protobuf.Root;
const Type = $protobuf.Type;
const Field = $protobuf.Field;

CURRENCY_SERVICE_ID = 1;
VOTING_SERVICE_ID = 2;
TRANSACTIONS_URL = 'http://127.0.0.1:8000/api/explorer/v1/transactions';

// For generate new one use
const keyPair = Exonum.keyPair();
//keyPair = {
//publicKey: 'c97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
//secretKey:
//'a4033ba9c9ee6098f32a6f904f19a7feea155c36c5116d578a8399f3d2eea67dc97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
//};

let CreateSchema = new Type('CreateSchema').add(new Field('name', 1, 'string'));

const CreateWalletTx = Exonum.newTransaction({
  author: keyPair.publicKey,
  service_id: CURRENCY_SERVICE_ID,
  message_id: 0,
  schema: CreateSchema,
});

const CreateCandidatetTx = Exonum.newTransaction({
  author: keyPair.publicKey,
  service_id: VOTING_SERVICE_ID,
  message_id: 0,
  schema: CreateSchema,
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

function createCandidate(name, keyPair) {
  const data = {
    name: name,
  };

  CreateCandidatetTx.send(TRANSACTIONS_URL, data, keyPair.secretKey)
    .then(r => console.log(r))
    .catch(e => console.log(e));
}

//createWallet('John', keyPair);
createCandidate('John', keyPair);
