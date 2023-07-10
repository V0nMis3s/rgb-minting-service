# RGB - Minting Service CLI

## Overview 

This project provides a command-line interface (CLI) for a small RGB minting service based on the RGB protocol. It is designed to facilitate the creation and the transfer of NFTs on the Bitcoin regtest network.

The service accepts a definition of the assets and a blinded UTXO. The newly minted assets ownership is then transferred to the owner of the specified UTXO.

For more in-depth information about the RGB protocol, please visit the [official RGB website](https://rgb.info/).

## Requirements

Before you begin, ensure you have the following installed on your local machine:

- Rust: Use [rustup](https://rustup.rs/) to manage your Rust installation.
- Docker: Follow the [official installation guide](https://docs.docker.com/get-docker/).
- Docker Compose: The instructions can be found on the [official documentation page](https://docs.docker.com/compose/install/).

## Usage

To try the service by minting RGB121 assets execute:
```sh
cargo run mint-rgb121 "Test" "Test description" 1 "sample.png" "txob1y3w8h9n4v4tkn37uj55dvqyuhvftrr2cxecp4pzkhjxjc4zcfxtsmdt2vf"
```

To try the service minting RGB20 assets execute:
```sh
cargo run mint-rgb20 "Test" "TST" 100 "txob1y3w8h9n4v4tkn37uj55dvqyuhvftrr2cxecp4pzkhjxjc4zcfxtsmdt2vf"
```

For help execute:
```sh
cargo run help
```

```sh
cargo run mint-rgb121 --help
```

```sh
cargo run mint-rgb20 --help
```
