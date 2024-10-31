use std::{error::Error, str::FromStr};

use clap::{Parser, ValueEnum};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
};
use solana_pubkey::Pubkey;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    signature::Signature,
};

#[derive(Clone, Copy, ValueEnum, Debug)]
enum Network {
    Mainnet,
    Devnet,
    Testnet,
}

impl Network {
    fn to_string(&self) -> String {
        match self {
            Network::Testnet => "https://testnet.dev2.eclipsenetwork.xyz/".to_string(),
            Network::Devnet => "https://staging-rpc.dev2.eclipsenetwork.xyz".to_string(),
            Network::Mainnet => "https://mainnetbeta-rpc.eclipse.xyz/".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    network: Network,
    pubkey: Pubkey,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let args = Args::parse();
    let client = RpcClient::new(args.network.to_string());
    let mut last = None;
    let mut total = 0;
    loop {
        log::info!("Searching before {:?}", last);
        let signatures = client
            .get_signatures_for_address_with_config(
                &args.pubkey,
                GetConfirmedSignaturesForAddress2Config {
                    before: last,
                    until: None,
                    limit: None,
                    commitment: Some(CommitmentConfig {
                        commitment: CommitmentLevel::Finalized,
                    }),
                },
            )
            .await?;

        log::info!("Found {}", signatures.len());

        total += signatures.len();

        if signatures.len() == 0 {
            break;
        }

        last = signatures
            .last()
            .map(|x| Signature::from_str(&x.signature).unwrap());
    }

    println!("Total: {}", total);

    Ok(())
}
