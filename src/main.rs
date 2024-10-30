use std::{error::Error, str::FromStr};

use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
};
use solana_pubkey::pubkey;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    signature::Signature,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = RpcClient::new("https://testnet.dev2.eclipsenetwork.xyz/".to_string());
    let program_id = pubkey!("6Qa4uMu2Ama76jBQ86x24QUrArxZUL26rqgARveR7ZSN");
    let mut last = None;
    let mut total = 0;
    loop {
        println!("Searching before {:?}", last);
        let signatures = client
            .get_signatures_for_address_with_config(
                &program_id,
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

        println!("Found {}", signatures.len());

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
