use anyhow::{anyhow, Error};
use clap::{Parser, ValueEnum};
use fehler::throws;
use serde_json::from_str;
use solana_sdk::{signature::Keypair, signer::keypair::read_keypair_file};
use std::{fmt::Debug, path::PathBuf};

#[derive(Debug, Clone, Parser)]
#[clap(name = "sbjc", about = "Convert between json format and base58 format")]
struct Cli {
    #[arg(long, short)]
    json: Option<String>,

    #[arg(long, short)]
    base58: Option<String>,

    #[arg(long, short, default_value = "naked")]
    output: OutputFormat,
}

#[derive(Debug, Clone, Copy, Parser, ValueEnum)]
pub enum OutputFormat {
    Naked,
    JSON,
}

trait Output {
    fn output(&self, format: OutputFormat);
}

impl<'a> Output for &'a [u8] {
    fn output(&self, format: OutputFormat) {
        match format {
            OutputFormat::Naked => {
                println!("{:?}", self);
            }
            OutputFormat::JSON => {
                println!(r#"{{"value": {:?}}}"#, self);
            }
        }
    }
}

impl<'a> Output for &'a str {
    fn output(&self, format: OutputFormat) {
        match format {
            OutputFormat::Naked => {
                println!("{}", self);
            }
            OutputFormat::JSON => {
                println!(r#"{{"value": {:?}}}"#, self);
            }
        }
    }
}

#[throws(Error)]
pub fn load_keypair(src: &str) -> Keypair {
    let maybe_keypair = shellexpand::full(&src)
        .map_err(|e| anyhow!(e))
        .and_then(|path| -> std::result::Result<_, Error> {
            Ok(PathBuf::from(&*path).canonicalize()?)
        })
        .and_then(|path| read_keypair_file(&path).map_err(|_| anyhow!("Cannot read keypair")));

    match maybe_keypair {
        Ok(keypair) => keypair,
        Err(_) => Keypair::from_bytes(&bs58::decode(src).into_vec()?)?,
    }
}

#[throws(Error)]
fn main() {
    let cli = Cli::parse();

    if cli.json.is_none() && cli.base58.is_none() {
        eprintln!("Either --json or --base58 should be specified");
    }

    if let Some(json) = cli.json {
        if let Ok(wallet) = load_keypair(&json) {
            wallet.to_base58_string().as_str().output(cli.output);
            return;
        }

        let array: Vec<_> = from_str(&json)?;
        bs58::encode(&array)
            .into_string()
            .as_str()
            .output(cli.output);
        return;
    }

    if let Some(b58) = cli.base58 {
        bs58::decode(&b58).into_vec()?.as_slice().output(cli.output);
        return;
    }
}
