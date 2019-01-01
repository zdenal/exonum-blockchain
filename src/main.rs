extern crate exonum;
extern crate exonum_blockchain;

use exonum::blockchain::{GenesisConfig, ValidatorKeys};
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::storage::MemoryDB;

use exonum_blockchain::currency::service::CurrencyService;

fn node_config() -> NodeConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_address = "0.0.0.0:8000".parse().unwrap();
    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        ..Default::default()
    };

    let peer_address = "0.0.0.0:2000".parse().unwrap();

    // Return this value from `node_config` function
     NodeConfig {
        listen_address: peer_address,
        service_public_key,
        service_secret_key,
        consensus_public_key,
        consensus_secret_key,
        genesis,
        external_address: peer_address,
        network: Default::default(),
        connect_list: Default::default(),
        api: api_cfg,
        mempool: Default::default(),
        services_configs: Default::default(),
        database: Default::default(),
     }

}

fn main() {
    exonum::helpers::init_logger().unwrap();

    println!("Creating in-memory database...");
    let node = Node::new(
        MemoryDB::new(),
        vec![Box::new(CurrencyService)],
        node_config(),
        None,
    );
    println!("Starting a single node...");
    println!("Blockchain is ready for transactions!");
    node.run().unwrap();
}
