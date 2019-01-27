#![allow(bare_trait_objects)]
#![allow(renamed_and_removed_lints)]

pub use self::cryptocurrency::{TxCreateWallet, TxTransfer, Wallet};

include!(concat!(env!("OUT_DIR"), "/currency_protobuf_mod.rs"));

use exonum::proto::schema::*;
