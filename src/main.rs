mod cli;
mod setup;
mod telemetry;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use clap::Parser;
use rgb_lib::{
    generate_keys, BitcoinNetwork,
    wallet::{DatabaseType, Recipient, Wallet, WalletData}
};
use tracing::{error, info};

const DATA_DIR: &str = "./data";
const ELECTRUM_URL: &str = "tcp://localhost:50001";

fn main() {
    // Setup cli
    let cli = cli::Cli::parse();

    // Setup tracing
    let subscriber = telemetry::get_subscriber(
        "minting-service".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // Setup services
    setup::start_services();

    fs::create_dir_all(DATA_DIR).unwrap();

    match cli.command {
        cli::Commands::MintCollectible {
            name,
            description,
            amount,
            file_path,
            blinded_utxo
        } => {
            // RGB Minter Wallet setup
            info!("Setting up Minter Wallet");
            let minter_keys = generate_keys(BitcoinNetwork::Regtest);
            let minter_wallet_data = WalletData {
                data_dir: DATA_DIR.parse().unwrap(),
                bitcoin_network: BitcoinNetwork::Regtest,
                database_type: DatabaseType::Sqlite,
                pubkey: minter_keys.xpub,
                mnemonic: Some(minter_keys.mnemonic)
            };
            let mut minter_wallet = Wallet::new(minter_wallet_data).unwrap();
            let online_data = minter_wallet
                .go_online(true, ELECTRUM_URL.parse().unwrap())
                .unwrap();
            let minter_address = minter_wallet.get_address();
            setup::fund_wallet(&minter_address, "0.001");
            setup::mine();
            minter_wallet
                .create_utxos(
                    online_data.clone(),
                    true,
                    Some(5),
                    None,
                    1.5)
                .unwrap();

            // Asset definition setup
            info!("Value of asset name: {}, description: {}, amount: {}, file path: {}",
                name, description, amount, file_path);

            let path = Path::new(&file_path);
            if !path.exists() {
                error!("Missing image file at the specified path: {:?}", path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
                std::process::exit(1);
            }
            info!("Value of the blinded UTXO: {}", blinded_utxo);

            // RGB NFT issuance
            info!("Issuing Collectibles");
            let rgb121_asset = minter_wallet.issue_asset_rgb121(
                online_data.clone(),
                name.to_string(),
                Some(description.to_string()),
                0,
                vec![amount],
                None,
                Some(file_path.to_string())
            ).unwrap_or_else(|e| {
                error!("Encountered error while issuing the RGB121: {:?}", e);
                std::process::exit(1);
            });

            let wallet_rgb121_assets = minter_wallet
                .list_assets(vec![])
                .expect("Failed to list RGB121 assets")
                .rgb121
                .unwrap();

            info!("RGB121 assets: {:#?}", wallet_rgb121_assets);

            // RGB NFT transfer
            let consignment_endpoints =
                vec!["rgbhttpjsonrpc:http://localhost:3000/json-rpc".to_string()];

            let recipient_map_rgb121 = HashMap::from([(
                rgb121_asset.asset_id.to_string(),
                vec![Recipient {
                    amount,
                    blinded_utxo,
                    consignment_endpoints,
                }],
            )]);

            let txid = minter_wallet.send(
                online_data,
                recipient_map_rgb121,
                false,
                1.5
            ).unwrap_or_else(|e| {
                error!("Encountered error while transferring the RGB121: {:?}", e);
                std::process::exit(1);
            });
            assert!(!txid.is_empty());
            info!("RGB121 txid: {:#?}", txid);

            // Mining a block for progressing transfer to a final state
            setup::mine();
        }
    }

    // Services teardown
    setup::stop_services();
}
