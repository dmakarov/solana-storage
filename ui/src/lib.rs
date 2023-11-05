#![allow(non_snake_case)]

use async_process::{Command, Stdio};
use dioxus::prelude::*;
use futures::{io::BufReader, prelude::*};

pub fn Client(cx: Scope) -> Element {
    render! {"Client application"}
}

pub fn Validator(cx: Scope) -> Element {
    let validator_log_line = use_state(cx, || String::new());
    let _vlc: &Coroutine<()> = use_coroutine(cx, |_rx| {
        let validator_log_line = validator_log_line.to_owned();
        async move {
            if let Ok(mut child) = Command::new("solana-test-validator")
                .env("RUST_LOG", "info,solana_accounts_db=warn,solana_core=warn,solana_metrics=warn,solana_poh=warn,solana_runtime::bank=warn")
                .arg("-C").arg("config.yml")
                .arg("-l").arg("test-ledger")
                .stdout(Stdio::piped())
                .spawn()
            {
                let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
                while let Some(line) = lines.next().await {
                    validator_log_line.set(format!("{}", line.unwrap()));
                }
            } else {

                //child.kill()?;

            }
        }
    });
    render! { format!("{}\n", validator_log_line.get()) }
}

pub fn Commands(cx: Scope) -> Element {
    render! {"Commands"}
    //render! { (0..1000).map(|i| rsx!(li { "{i}" })) }
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
