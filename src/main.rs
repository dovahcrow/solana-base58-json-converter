use anyhow::{anyhow, Error};
use fehler::throws;
use serde_json::from_str;
use solana_sdk::{signature::Keypair, signer::keypair::read_keypair_file};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "transact", about = "Making transactions to the GFX Swap")]
struct Opt {
    #[structopt(long, short)]
    json: Option<String>,

    #[structopt(long, short)]
    base58: Option<String>,
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
    let opt = Opt::from_args();

    if opt.json.is_none() && opt.base58.is_none() {
        eprintln!("Either --json or --base58 should be specified");
    }

    if let Some(json) = opt.json {
        if let Ok(wallet) = load_keypair(&json) {
            println!("{}", wallet.to_base58_string());
            return;
        }

        let array: Vec<_> = from_str(&json)?;
        println!("{}", bs58::encode(&array).into_string());
        return;
    }

    if let Some(b58) = opt.base58 {
        println!("{:?}", bs58::decode(&b58).into_vec()?);
        return;
    }
}
