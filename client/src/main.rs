use clap::{crate_description, crate_name, crate_version, value_t_or_exit, Arg, SubCommand};
use client;
use solana_sdk::signature::Signer;

fn main() {
    let version = format!("{}", crate_version!());
    let args = std::env::args().collect::<Vec<_>>();
    let matches = clap::Command::new(crate_name!())
        .about(crate_description!())
        .version(version.as_str())
        .arg(
            Arg::new("config")
                .long("config")
                .short('C')
                .takes_value(true)
                .value_name("CONFIG")
                .help("Config filepath"),
        )
        .arg(
            Arg::new("keypair")
                .long("keypair")
                .short('k')
                .takes_value(true)
                .value_name("KEYPAIR")
                .help("Filepath or URL to a keypair"),
        )
        .arg(
            Arg::new("url")
                .long("url")
                .short('u')
                .takes_value(true)
                .value_name("URL_OR_MONIKER")
                .help("URL for Solana's JSON RPC or moniker (or their first letter): [mainnet-beta, testnet, devnet, localhost]"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .takes_value(false)
                .help("Use verbose output"),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("create color object")
                .arg(
                    Arg::with_name("red")
                        .index(1)
                        .value_name("BYTE")
                        .takes_value(true)
                        .multiple(false)
                        .required(true)
                        .help("red color components"),
                )
                .arg(
                    Arg::with_name("green")
                        .index(2)
                        .value_name("BYTE")
                        .takes_value(true)
                        .multiple(false)
                        .required(true)
                        .help("green color components"),
                )
                .arg(
                    Arg::with_name("blue")
                        .index(3)
                        .value_name("BYTE")
                        .takes_value(true)
                        .multiple(false)
                        .required(true)
                        .help("blue color components"),
                )
        )
        .subcommand(
            SubCommand::with_name("transfer")
                .about("transfer color object between accounts")
        )
        .get_matches_from(args);
    let config: Option<String> = matches.value_of_t("config").ok();
    let keypair: String = matches.value_of_t_or_exit("keypair");
    let url: Option<String> = matches.value_of_t("url").ok();
    let connection = client::client::establish_connection(&url, &config).unwrap();
    println!(
        "Connected to remote solana node running version ({}).",
        connection.get_version().unwrap()
    );
    let balance_requirement = client::client::get_balance_requirement(&connection).unwrap();
    println!(
        "{:18} lamports are required for this transaction.",
        balance_requirement
    );
    let payer = client::utils::get_payer(&config).unwrap();
    let payer_balance = client::client::get_payer_balance(&payer, &connection).unwrap();
    println!("{} lamports are owned by payer.", payer_balance);
    if payer_balance < balance_requirement {
        let request = balance_requirement - payer_balance;
        println!(
            "Payer does not own sufficent lamports. Airdropping {} lamports.",
            request
        );
        client::client::request_airdrop(&payer, &connection, request).unwrap();
    }
    let program = client::client::get_program(&keypair, &connection).unwrap();
    match matches.subcommand() {
        Some(("create", arg_matches)) => {
            let red = value_t_or_exit!(arg_matches, "red", u8);
            let green = value_t_or_exit!(arg_matches, "green", u8);
            let blue = value_t_or_exit!(arg_matches, "blue", u8);
            client::client::create_object_account(&payer, "sender", &program, &connection).unwrap();
            let object_account_pubkey =
                client::utils::get_object_account_public_key(&payer.pubkey(), "sender", &program.pubkey()).unwrap();
            client::client::dump_account("Object account created", &object_account_pubkey, &connection).unwrap();
            client::client::create_color_object(
                &payer,
                "sender",
                red,
                green,
                blue,
                &program,
                &connection,
            ).unwrap();
            client::client::dump_account("Color object created", &object_account_pubkey, &connection).unwrap();
        }
        Some(("transfer", _arg_matches)) => {
            let sender_account_pubkey =
                client::utils::get_object_account_public_key(&payer.pubkey(), "sender", &program.pubkey()).unwrap();
            client::client::dump_account("Sender account", &sender_account_pubkey, &connection).unwrap();
            client::client::create_object_account(&payer, "receiver", &program, &connection).unwrap();
            let receiver_pubkey =
                client::utils::get_object_account_public_key(&payer.pubkey(), "receiver", &program.pubkey()).unwrap();
            client::client::dump_account("Receiver account created", &receiver_pubkey, &connection).unwrap();
            client::client::transfer_color_object(
                &payer,
                "receiver",
                "sender",
                &program,
                &connection,
            ).unwrap();
            client::client::dump_account("Sender account after transfer", &sender_account_pubkey, &connection).unwrap();
            client::client::dump_account("Receiver account after transfer", &receiver_pubkey, &connection).unwrap();
        }
        _ => unreachable!(),
    };
    let payer_balance = client::client::get_payer_balance(&payer, &connection).unwrap();
    println!("{} lamports are owned by payer.", payer_balance);
}
