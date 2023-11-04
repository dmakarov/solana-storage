#![allow(non_snake_case)]

use async_process::{Command, Stdio};
use dioxus::prelude::*;
use futures::{io::BufReader, prelude::*};

pub fn Client(cx: Scope) -> Element {
    render! {"Client application"}
}

pub fn Validator(cx: Scope) -> Element {
    render! { (0..1000).map(|i| rsx!(li { "{i}" })) }
    /*
    let validator_log = use_future(cx, (), |_| run_test_validator());
    match validator_log.value() {
        Some(Ok(list)) => render! {
            for line in list {
                render! { "{line}" }
            }
        },
        Some(Err(e)) => render! {
            "Failed to start validator {e}"
        },
        None => render! {
            "Starting validator"
        },
    }
    */
}

pub fn Commands(cx: Scope) -> Element {
    render! {"Commands"}
}

pub async fn compile_smart_contract() -> std::io::Result<()> {
    println!("RUST_LOG=info cargo build-sbf --manifest-path=program/Cargo.toml");
    let mut child = Command::new("cargo")
        .env("RUST_LOG", "info")
        .arg("build-sbf")
        .arg("--manifest-path=program/Cargo.toml")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        println!("{}", line?);
    }
    Ok(())
}

pub fn deploy_smart_contract() {
    println!("solana program deploy --use-quic -k test.json -u localhost program/target/deploy/storage.so");
}

pub fn run_client_with_options() {
    println!("cargo r -p client -- -C config.yml -k program/target/deploy/storage-keypair.json -u localhost create 255 255 0");
}

pub async fn run_test_validator() -> std::io::Result<Vec<String>, > {
    //println!("solana-test-validator -C config.yml -l test-ledger");
    let mut child = Command::new("solana-test-validator")
        .arg("-C").arg("config.yml")
        .arg("-l").arg("test-ledger")
        .arg("-q")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    //child.kill()?;

    let mut collected_log: Vec<String> = vec![];

    while let Some(line) = lines.next().await {
        collected_log.push(format!("{}", line?));
    }
    Ok(collected_log)
}
