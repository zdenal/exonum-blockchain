const Exonum = require('exonum-client');
const voting = require('./lib/voting');
const chain = require('./lib/chain');
const currency = require('./lib/currency');

// For generate new one use
const candidateKeyPair = Exonum.keyPair();
const voterKeyPair = Exonum.keyPair();

authorityKeyPair = {
  publicKey: 'c97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
  secretKey:
    'a4033ba9c9ee6098f32a6f904f19a7feea155c36c5116d578a8399f3d2eea67dc97923414103b2f40ca1fa798bca4d514dc07813e7ab720859bc74848e491550',
};

//currency.createWallet('John', authorityKeyPair);
//voting.createCandidate('Alois', candidateKeyPair, authorityKeyPair);
//voting.vote(candidateKeyPair, voterKeyPair);
//voting.getCandidates();
//chain.getBlocks();
chain.getHeight();
//chain.getBlockTransactions(43);
