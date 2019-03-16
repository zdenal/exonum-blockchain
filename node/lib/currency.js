const Exonum = require('exonum-client');
const axios = require('axios');
const cTable = require('console.table');
const $protobuf = require('protobufjs/light');
const Root = $protobuf.Root;
const Type = $protobuf.Type;
const Field = $protobuf.Field;

CURRENCY_SERVICE_ID = 1;
TRANSACTIONS_URL = 'http://127.0.0.1:8000/api/explorer/v1/transactions';

const CreateSchema = new Type('CreateSchema').add(
  new Field('name', 1, 'string'),
);

function CreateWalletTx(author) {
  return Exonum.newTransaction({
    author: authorityKeyPair.publicKey,
    service_id: CURRENCY_SERVICE_ID,
    message_id: 0,
    schema: CreateSchema,
  });
}

exports.createWallet = function(name, keyPair) {
  const data = {
    name: name,
  };

  //const signature = CreateWalletTx(keyPair).sign(keyPair.secretKey, data);
  //console.log('signature: ', signature);

  CreateWalletTx(keyPair)
    .send(TRANSACTIONS_URL, data, keyPair.secretKey)
    .then(r => console.log(r))
    .catch(e => console.log(e));
};
