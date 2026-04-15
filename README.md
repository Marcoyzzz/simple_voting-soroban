# Soroban Simple Voting Smart Contract

## Description
The **Soroban Simple Voting Contract** is a decentralized application (dApp) built on the Stellar network using the Soroban smart contract platform. This contract provides a lightweight, efficient, and complete CRUD (Create, Read, Update, Delete) system to conduct transparent elections or polls. 

Instead of storing all data in a single array, it utilizes dynamic storage keys. Each candidate's name acts as a unique storage key mapped directly to their accumulated vote count, ensuring optimized state management and scalability on the blockchain.

## Features
* **Vote (Create/Update):** Cast a vote. Voting for a new candidate automatically registers them on the ledger.
* **Read Votes (Read):** Instantly retrieve the current total votes for any given candidate.
* **Reset Votes (Update):** Reset a candidate's vote count back to zero without deleting their record.
* **Delete Candidate (Delete):** Completely remove a candidate's record and their vote data from the blockchain to optimize storage and save gas fees.

## Testnet Contract ID
This contract has been deployed to the Stellar Futurenet/Testnet for testing purposes. You can interact with it using the following Contract ID:

**Contract ID:** `[YOUR_TESTNET_CONTRACT_ID_HERE]` *(e.g., CC5Y7X...)*

## Prerequisites
To interact with this contract, ensure you have the following installed:
* [Rust](https://www.rust-lang.org/tools/install)
* [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
* A funded Stellar Testnet account.

## How to Interact (Using Soroban CLI)

### 1. Cast a Vote
Add 1 vote to a specific candidate (e.g., "Alice").
```bash
soroban contract invoke \
  --id [YOUR_TESTNET_CONTRACT_ID_HERE] \
  --source-account [YOUR_ACCOUNT_ALIAS] \
  --network testnet \
  -- \
  vote \
  --candidate "Alice"

  https://stellar.expert/explorer/testnet/account/GBTLTQCWYPNUWR5IWDMOPTHN435OPUA3ONFYA3B57RK7IPZTPWWYPGHP