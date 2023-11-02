pub fn compile_smart_contract() {
    println!("RUST_LOG=info cargo build-sbf --manifest-path=program/Cargo.toml");
}

pub fn deploy_smart_contract() {
    println!("solana program deploy --use-quic -k test.json -u localhost program/target/deploy/storage.so");
}

pub fn run_client_with_options() {
    println!("cargo run --manifest-path=client/Cargo.toml -- -C config.yml -k program/target/deploy/storage-keypair.json -u localhost create 255 255 0");
}

pub fn run_test_validator() {
    println!("solana-test-validator -C ~/work/try/simple-solana-program/config.yml -l ~/work/try/simple-solana-program/test-ledger");
}
