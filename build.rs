extern crate exonum_build;

use exonum_build::{get_exonum_protobuf_files_path, protobuf_generate};

fn main() {
    let exonum_protos = get_exonum_protobuf_files_path();
    protobuf_generate(
        "src/currency/proto",
        &["src/currency/proto", &exonum_protos],
        "currency_protobuf_mod.rs",
    );
    protobuf_generate(
        "src/voting/proto",
        &["src/voting/proto", &exonum_protos],
        "voting_protobuf_mod.rs",
    );
}
