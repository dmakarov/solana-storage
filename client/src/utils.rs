use crate::{Error, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use yaml_rust::YamlLoader;

/// The schema for object storage in object accounts. This is what
/// is serialized into the account and updated when objects are manipulated.
#[derive(BorshSerialize, BorshDeserialize)]
struct ObjectSchema<T> {
    exists: bool,
    value: T,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct ColorObject {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Parses and returns the Solana yaml config on the system.
pub fn get_config(config: &Option<String>) -> Result<yaml_rust::Yaml> {
    let path = match config {
        Some(path) => std::path::PathBuf::from(path),
        None => {
            match home::home_dir() {
                Some(mut path) => {
                    path.push(".config/solana/cli/config.yml");
                    path
                }
                None => {
                    return Err(Error::ConfigReadError(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "failed to locate homedir and thus can not locoate solana config",
                    )));
                }
            }
        }
    };
    let config = std::fs::read_to_string(path).map_err(|e| Error::ConfigReadError(e))?;
    let mut config = YamlLoader::load_from_str(&config)?;
    match config.len() {
        1 => Ok(config.remove(0)),
        l => Err(Error::InvalidConfig(format!(
            "expected one yaml document got ({})",
            l
        ))),
    }
}

/// Gets the RPC url for the cluster that this machine is configured
/// to communicate with.
pub fn get_rpc_url(config: &Option<String>) -> Result<String> {
    let config = get_config(config)?;
    match config["json_rpc_url"].as_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(Error::InvalidConfig(
            "missing `json_rpc_url` field".to_string(),
        )),
    }
}

/// Gets the "payer" or local solana wallet that has been configured
/// on the machine.
pub fn get_payer(config: &Option<String>) -> Result<Keypair> {
    let config = get_config(config)?;
    if let Some(path) = config["keypair_path"].as_str() {
        read_keypair_file(path).map_err(|e| {
            Error::InvalidConfig(format!("failed to read keypair file ({}): ({})", path, e))
        })
    } else {
        Err(Error::InvalidConfig("missing `keypair_path` field".to_string()))
    }
}

/// Derives and returns the object account public key
/// for a given PAYER, SEED, PROGRAM combination.
pub fn get_object_account_public_key(
    payer: &Pubkey,
    seed: &str,
    program: &Pubkey,
) -> Result<Pubkey> {
    Ok(Pubkey::create_with_seed(payer, seed, program)?)
}

/// Determines and reports the size of object.
pub fn get_object_data_size() -> Result<usize> {
    let encoded = ObjectSchema::<ColorObject> {
        exists: false,
        value: ColorObject { red: 0, green: 0, blue: 0 },
    }
    .try_to_vec()
    .map_err(|e| Error::SerializationError(e))?;
    Ok(encoded.len())
}
