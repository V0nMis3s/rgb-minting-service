# RGB - NFT Minting Service CLI

## Overview 

This project provides a command-line interface (CLI) for a small non-fungible token (NFT) minting service based on the RGB protocol. It is designed to facilitate the creation and the transfer of NFTs on the Bitcoin regtest network.

The service accepts a definition of an NFT token and a blinded UTXO. The newly minted assets ownership is then transferred to the owner of the specified UTXO.

For more in-depth information about the RGB protocol, please visit the [official RGB website](https://rgb.tech/).

## Requirements

Before you begin, ensure you have the following installed on your local machine:

- Rust: Use [rustup](https://rustup.rs/) to manage your Rust installation.
- Docker: Follow the [official installation guide](https://docs.docker.com/get-docker/).
- Docker Compose: The instructions can be found on the [official documentation page](https://docs.docker.com/compose/install/).

## Usage

To try the service execute:

```sh
cargo run mint-collectible "Test" "Test description" 1 "sample.png" "txob1y3w8h9n4v4tkn37uj55dvqyuhvftrr2cxecp4pzkhjxjc4zcfxtsmdt2vf"
```
