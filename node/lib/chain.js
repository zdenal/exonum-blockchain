const axios = require('axios');
const cTable = require('console.table');

exports.getBlocks = async function() {
  const response = await axios.get(
    'http://127.0.0.1:8000/api/services/chain/v1/blocks',
  );
  console.table(response.data.filter(block => block.tx_count > 0));
};

exports.getBlockTransactions = async function(height) {
  const response = await axios.get(
    'http://127.0.0.1:8000/api/services/chain/v1/block_transactions',
    {
      params: {
        height: height,
      },
    },
  );
  console.log(response.data);
};
