use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    version = "0.2.0",
    author = "Giwdul Sesimnov <giwdulsesimnov@gmail.com>",
    about = "RGB Minting Service CLI"
)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Mint an NFT passing the asset definition and a blinded UTXO
    MintRGB121{
        #[clap(default_value_t = String::from("Test"))]
        name: String,
        #[clap(default_value_t = String::from("Test description"))]
        description: String,
        #[clap(default_value_t = 1)]
        amount: u64,
        #[clap(default_value_t = String::from("sample.png"))]
        file_path: String,
        #[clap(
            default_value_t =
                String::from("txob1y3w8h9n4v4tkn37uj55dvqyuhvftrr2cxecp4pzkhjxjc4zcfxtsmdt2vf")
        )]
        blinded_utxo: String,
    },
    /// Mint an RGB20 passing the asset definition and a blinded UTXO
    MintRGB20{
        #[clap(default_value_t = String::from("Test"))]
        name: String,
        #[clap(default_value_t = String::from("TST"))]
        ticker: String,
        #[clap(default_value_t = 100)]
        amount: u64,
        #[clap(
            default_value_t =
                String::from("txob1y3w8h9n4v4tkn37uj55dvqyuhvftrr2cxecp4pzkhjxjc4zcfxtsmdt2vf")
        )]
        blinded_utxo: String,
    }
}