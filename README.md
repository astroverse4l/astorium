# Astorium Project

This project contains the source code for the Astorium DeFi project on the Solana blockchain.

## Project Structure

- `program/`: This directory contains the source code for the Solana staking program.
- `token/`: This directory contains the script to create the Astorium SPL-Token with metadata.

## How to Get Started

### 1. Prerequisites

- Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
- Install the [Solana CLI](https://docs.solana.com/cli/install).
- Install [Node.js and npm](https://nodejs.org/en/download/).

### 2. Create the Astorium SPL-Token

The token creation script uses the Metaplex Token Metadata standard to create the token with a name, symbol, and image.

First, update the `token/metadata.json` file with your desired token details. Make sure to replace the placeholder image URL with a valid one.

Then, run the following commands:

```bash
cd token
npm install
npm start
```

This will run the `index.ts` script, which creates the token. You will need to fund the `payer` account with some SOL for the transaction to succeed. You can airdrop some SOL to the payer's address using the Solana CLI:

```bash
solana airdrop 1 <PAYER_ADDRESS> --url https://api.devnet.solana.com
```

### 3. Build and Deploy the Staking Program

The staking program allows users to stake their Astorium tokens.

To build the program:
```bash
cd program
cargo build-sbf
```

This will create a `astorium_program.so` file in the `program/target/deploy` directory.

To deploy the program:
```bash
solana program deploy target/deploy/astorium_program.so
```

### 4. Interacting with the Staking Program

To interact with the staking program, you will need to build a frontend application that sends transactions to the program's instructions (`InitializeStakeAccount`, `Stake`, `Unstake`). This is a common next step after deploying the program.

## Multi-Chain Compatibility

Solana is not EVM-compatible, which means that smart contracts written for Solana cannot be directly deployed on other blockchains like Ethereum. To achieve cross-chain functionality, you will need to use a "bridge" like [Wormhole](https://wormholenetwork.com/) or [Allbridge](https://allbridge.io/).

**Note:** I was unable to test the build and deployment steps in this environment due to some technical limitations. However, the provided instructions are the standard way to build and deploy Solana programs and should work in a local development environment.
