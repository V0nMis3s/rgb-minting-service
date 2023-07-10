mod cli;
mod setup;
mod telemetry;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use clap::Parser;
use rgb_lib::{
    generate_keys, BitcoinNetwork,
    wallet::{Assets, DatabaseType, Recipient, Wallet, WalletData}
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

    let amount: u64;
    let blinded_utxo: String;
    let rgb_asset: Assets;
    match cli.command {
        cli::Commands::MintRGB121 {
            name,
            description,
            amount: inner_amount,
            file_path,
            blinded_utxo: inner_blinded_utxo
        } => {
            amount = inner_amount;
            blinded_utxo = inner_blinded_utxo;
            // Asset definition setup
            info!("Value of asset name: {}, description: {}, amount: {}, file path: {}",
                name, description, amount, file_path);

            let path = Path::new(&file_path);
            if !path.exists() {
                error!("Missing image file at the specified path: {:?}",
                    path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
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

            rgb_asset = Assets {
                rgb20: None,
                rgb121: Some(vec![rgb121_asset])
            }
        },
        cli::Commands::MintRGB20 {
            name,
            ticker,
            amount: inner_amount,
            blinded_utxo: inner_blinded_utxo
        } => {
            amount = inner_amount;
            blinded_utxo = inner_blinded_utxo;
            // Asset definition setup
            info!("Value of asset name: {}, ticker: {}, amount: {}",
                name, ticker, amount);
            info!("Value of the blinded UTXO: {}", blinded_utxo);
            // RGB20 issuance
            info!("Issuing RGB20");
            let rgb20_asset = minter_wallet.issue_asset_rgb20(
                online_data.clone(),
                ticker.to_string(),
                name.to_string(),
                0,
                vec![amount],
            ).unwrap_or_else(|e| {
                error!("Encountered error while issuing the RGB20: {:?}", e);
                std::process::exit(1);
            });

            rgb_asset = Assets {
                rgb20: Some(vec![rgb20_asset]),
                rgb121: None
            }
        }

    }

    // Verify issued assets
    let wallet_rgb_assets = minter_wallet
        .list_assets(vec![])
        .expect("Failed to list RGB assets");

    info!("RGB20 assets: {:#?}, RGB121 assets: {:#?}",
        wallet_rgb_assets.rgb20.unwrap(), wallet_rgb_assets.rgb121.unwrap());

    // RGB NFT transfer
    let consignment_endpoints =
        vec!["rgbhttpjsonrpc:http://localhost:3000/json-rpc".to_string()];

    let asset_id: String;

    match (&rgb_asset.rgb20, &rgb_asset.rgb121) {
        (Some(rgb20), None) => {
            asset_id = rgb20.get(0).unwrap().asset_id.to_string()
        },
        (None, Some(rgb121)) => {
            asset_id = rgb121.get(0).unwrap().asset_id.to_string()
        }
        _ => {
            error!("No Asset minted. Unable to proceed with the transfer");
            std::process::exit(1)
        }
    }

    let recipient_map_rgb = HashMap::from([(
        asset_id,
        vec![Recipient {
            amount,
            blinded_utxo,
            consignment_endpoints,
        }],
    )]);

    let txid = minter_wallet.send(
        online_data,
        recipient_map_rgb,
        false,
        1.5
    ).unwrap_or_else(|e| {
        error!("Encountered error while transferring the asset: {:?}", e);
        std::process::exit(1);
    });
    assert!(!txid.is_empty());
    info!("RGB txid: {:#?}", txid);

    // Mining a block for progressing transfer to a final state
    setup::mine();

    // Services teardown
    setup::stop_services();
}
