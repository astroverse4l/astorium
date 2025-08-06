# Astorium Project

This project contains the source code for the Astorium DeFi project on the Solana blockchain.

## Project Structure

- `program/`: This directory contains the source code for the Solana program (smart contract).
- `token/`: This directory contains the script to create the Astorium SPL-Token.

## How to Build and Test

### 1. Prerequisites

- Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
- Install the [Solana CLI](https://docs.solana.com/cli/install).
- Install [Node.js and npm](https://nodejs.org/en/download/).

### 2. Build the Solana Program

```bash
cd program
cargo build-sbf
```

This will create a `astorium_program.so` file in the `program/target/deploy` directory.

### 3. Deploy the Solana Program

You can deploy the program to a local test validator or a public cluster.

To start a local test validator:
```bash
solana-test-validator
```

To deploy the program:
```bash
solana program deploy target/deploy/astorium_program.so
```

### 4. Create the Astorium SPL-Token

```bash
cd token
npm install
npm start
```

This will run the `index.ts` script, which creates the token. You will need to fund the `payer` account with some SOL for the transaction to succeed. You can airdrop some SOL to the payer's address using the Solana CLI:

```bash
solana airdrop 1 <PAYER_ADDRESS>
```

**Note:** I was unable to test the build and deployment steps in this environment due to some technical limitations. However, the provided instructions are the standard way to build and deploy Solana programs and should work in a local development environment.

## Multi-Chain Compatibility

The user requested that the coin be "tried across different chains". It is important to understand that Solana is not EVM-compatible, which means that smart contracts written for Solana cannot be directly deployed on other blockchains like Ethereum, Binance Smart Chain, or Polygon.

To achieve cross-chain functionality, you will need to use a "bridge". A bridge is a service that allows you to transfer tokens from one blockchain to another. Some popular bridges that support Solana are:

- [Wormhole](https://wormholenetwork.com/): A generic messaging protocol that connects to multiple chains including Ethereum, Terra, Binance Smart Chain, Polygon, Avalanche, and Oasis.
- [Allbridge](https://allbridge.io/): A simple, modern, and reliable way to transfer assets between different networks.

Using a bridge, you can "wrap" your Astorium token on other chains, allowing it to be traded and used in DeFi applications on those chains.
