#![allow(non_snake_case)]

use async_process::{Command, Stdio};
use dioxus::prelude::*;
use futures::{io::BufReader, prelude::*};

pub fn Client(cx: Scope) -> Element {
    let future_output = use_ref(cx, Vec::<String>::new);
    let deploy_output = use_ref(cx, Vec::<String>::new);
    let future = use_future(cx, (), |_| {
        let output = future_output.to_owned();
        start_subcommand(
            "cargo",
            vec![],
            &["build-sbf", "--manifest-path=program/Cargo.toml", "--", "-q"],
            output,
        )
    });
    let deploy = use_future(cx, (), |_| {
        let output = deploy_output.to_owned();
        start_subcommand(
            "solana",
            vec![],
            &["program", "deploy", "--use-quic", "-k", "test.json", "-u", "localhost", "program/target/deploy/storage.so"],
            output,
        )
    });
    let client = use_future(cx, (), |_| {
        let output = deploy_output.to_owned();
        start_subcommand(
            "cargo",
            vec![],
            &["r", "-p", "client", "--", "-C", "config.yml", "-k", "program/target/deploy/storage-keypair.json", "-u", "localhost", "create", "255", "255", "0"],
            output,
        )
    });

    let output_lock = future_output.read();
    let output_rendered = output_lock.iter().map(|x| {
        rsx!(format!("{}", x.clone()))
    });

    match future.value() {
        Some(_) => render! {
            div {
                height: "2px",
                justify_content: "center",
                "Client application"
            }
            div {
                output_rendered
            }
        },
        None => render! {
            div {
                height: "2px",
                justify_content: "center",
                "Client application"
            }
            div {
                "Compiling program..."
            }
        },
    }
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
                .stderr(Stdio::piped())
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
    render! {
        div {
            height: "2px",
            justify_content: "center",
            "Validator output"
        }
        div {
            height: "2px",
            format!("{}\n", validator_log_line.get())
        }
        div {
            display: "flex",
            flex_direction: "column",
            div { "1" }
            div { "2" }
            div { "3" }
            div { "4" }
            div { "5" }
        }
    }
}

pub fn Commands(cx: Scope) -> Element {
    render! {"Commands"}
}

async fn start_subcommand(command: &str, vars: Vec<(&str, &str)>, args: &[&str], output: UseRef<Vec<String>>) {
    let output = output.to_owned();
    if let Ok(mut child) = Command::new(command)
        .envs(vars.clone())
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
        let mut error = BufReader::new(child.stderr.take().unwrap()).lines();
        while let Some(line) = lines.next().await {
            output.with_mut(|v| v.push(format!("{}", line.unwrap())));
        }
        while let Some(line) = error.next().await {
            output.with_mut(|v| v.push(format!("{}", line.unwrap())));
        }
    } else {
        output.with_mut(|v| v.push(format!("Failed {:?} {} {:?}", &vars, command, args)));
    }
}
