# Bitcoin-like Cryptocurrency Implementation in Rust

## Table of Contents

1. [Introduction](#1-introduction)
2. [Project Goals](#2-project-goals)
3. [Architecture and Design Principles](#3-architecture-and-design-principles)
4. [File Structure Overview](#4-file-structure-overview)
5. [Module Details](#5-module-details)
   - [5.1 Core Module](#51-core-module)
     - [5.1.1 Block Submodule](#511-block-submodule-coreblock)
     - [5.1.2 Chain Submodule](#512-chain-submodule-corechain)
     - [5.1.3 Network Submodule](#513-network-submodule-corenetwork)
     - [5.1.4 Transaction Submodule](#514-transaction-submodule-coretransaction)
     - [5.1.5 CLI and RCP Submodules](#515-cli-and-rcp-submodules)
   - [5.2 Crypto Module](#52-crypto-module)
   - [5.3 Wallet Module](#53-wallet-module)
6. [Transaction Lifecycle](#6-transaction-lifecycle)
7. [Security Considerations](#7-security-considerations)
8. [Future Work and Roadmap](#8-future-work-and-roadmap)
9. [Conclusion](#9-conclusion)

## 1. Introduction

This project is a simplified Bitcoin-like cryptocurrency implemented in Rust. It’s designed as both an educational and experimental platform, demonstrating core blockchain principles such as decentralized ledger management, transaction validation, Proof of Work (PoW) consensus, and secure cryptographic operations. Though still in early development, it provides a solid foundation for understanding and extending blockchain technology.

## 2. Project Goals

- **Educational Purpose**: Serve as a demonstrative tool to understand how blockchain, PoW, and UTXO based transactions work.
- **Security & Verification**: Leverage cryptographic primitives (hashing, digital signatures, key management) to ensure transaction authenticity and integrity.
- **Modular Design**: Create a clean, modular architecture that can be incrementally expanded to include real networking and a user interface.
- **Future Scalability**: Lay the groundwork for enhancements such as a P2P networking layer, transaction fee mechanisms, and web interface integration.

## 3. Architecture and Design Principles

The implementation follows a modular structure that separates concerns across different domains:

- **Core Blockchain Logic**: Manages blocks, chain integrity, transaction pooling, and consensus rules.
- **Cryptographic Operations**: Handles secure hashing, key generation, and signature verification.
- **Wallet and Transaction Management**: Facilitates user interactions, from signing transactions to updating balances.
- **Network Simulation**: Emulates node-to-node interactions to validate and propagate data across a decentralized system.

Key design principles include:

- **Modularity**: Each functionality (block creation, transaction processing, cryptographic operations) is isolated into its own module.
- **Security**: Robust cryptographic techniques are used to prevent tampering and double-spending.
- **Scalability and Extensibility**: The design supports simulation of multiple nodes and is ready for potential transition to a distributed network.
- **Simplicity and Clarity**: The codebase is structured for clarity, easing the learning curve and facilitating future enhancements.

## 4. File Structure Overview

The project is organized as follows:

```
src
├── core
│  ├── block
│  │  ├── accessors.rs         # Functions to retrieve block properties
│  │  ├── mod.rs               # Block module interface
│  │  ├── pow.rs               # Proof of Work implementation (mining)
│  │  └── transactions.rs      # Block-specific transaction handling
│  ├── chain
│  │  ├── chain.rs             # Blockchain management and ledger state
│  │  ├── concensus.rs         # Consensus rules and conflict resolution
│  │  ├── mempool.rs           # Management of pending transactions
│  │  ├── mod.rs               # Chain module interface
│  │  └── transactions.rs      # Transaction integration within the chain
│  ├── cli                    # Command-line interface for direct blockchain interaction
│  ├── mod.rs                 # Entry point for core functionalities
│  ├── network
│  │  ├── block.rs             # Block propagation and network validation logic
│  │  ├── blockchain.rs        # Synchronization of blockchain states across nodes
│  │  ├── mod.rs               # Network module interface
│  │  └── transaction.rs       # Network transaction broadcasting and validation
│  ├── rcp
│  │  └── mod.rs               # Remote procedure call (API) interfaces (if applicable)
│  └── transaction
│     ├── accessors.rs         # Functions to access and parse transaction data
│     ├── mod.rs               # Transaction module interface
│     ├── transactions.rs      # Creation, signing, and validation of transactions
│     └── utxo.rs              # Unspent Transaction Output (UTXO) management
├── crypto
│  ├── hashing.rs             # Cryptographic hashing functions (e.g., SHA3-256)
│  ├── keys.rs                # Key generation and key pair management
│  ├── mod.rs                 # Crypto module entry point
│  └── signatures.rs          # Digital signature creation and verification routines
├── main.rs                  # Entry point for simulation and testing
└── wallet
   ├── mod.rs                # Wallet interface; handles key management and user operations
   └── transactions.rs       # Wallet-specific transaction construction and processing

```

## 5. Module Details

### 5.1 Core Module

The `core` module is the heart of the blockchain logic, encompassing several submodules:

### 5.1.1 Block Submodule (`core/block`)

- **Responsibilities**:
    - Construct blocks containing multiple transactions.
    - Mine blocks by solving the PoW challenge defined in `pow.rs`.
    - Provide accessor functions for querying block data.
- **Key Features**:
    - **Proof of Work (PoW)**: Implemented in `pow.rs`, it ensures that new blocks satisfy a predefined difficulty.
    - **Transaction Aggregation**: Blocks are assembled from validated transactions and include a coinbase transaction to reward miners.

### 5.1.2 Chain Submodule (`core/chain`)

- **Responsibilities**:
    - Manage the blockchain ledger, ensuring proper block order and integrity.
    - Initialize the genesis block and append new blocks after validation.
    - Resolve chain conflicts using consensus rules.
- **Key Components**:
    - **Chain Management (`chain.rs`)**: Handles the sequential addition of blocks.
    - **Consensus (`concensus.rs`)**: Contains rules for block validation and fork resolution.
    - **Mempool (`mempool.rs`)**: Temporarily holds unconfirmed transactions before block inclusion.

### 5.1.3 Network Submodule (`core/network`)

- **Responsibilities**:
    - Simulate decentralized network behavior by propagating transactions and blocks among nodes.
    - Synchronize blockchain copies across simulated nodes.
- **Key Components**:
    - **Block and Blockchain Propagation**: `block.rs` and `blockchain.rs` manage the reception and validation of new blocks.
    - **Transaction Handling**: `transaction.rs` deals with broadcasting and validating transactions across the network.

### 5.1.4 Transaction Submodule (`core/transaction`)

- **Responsibilities**:
    - Create and process transactions, ensuring each transaction adheres to UTXO based rules.
    - Manage transaction signatures and compute unique transaction IDs.
- **Key Components**:
    - **Transaction Creation and Validation (`transactions.rs`)**: Constructs transactions and performs signature checks.
    - **UTXO Management (`utxo.rs`)**: Tracks which outputs remain unspent to prevent double-spending.
    - **Accessors (`accessors.rs`)**: Facilitate data extraction from transaction structures.

### 5.1.5 CLI and RCP Submodules

- **CLI**: Provides a command-line interface for testing and administrative interactions with the blockchain.
- **RCP**: Offers potential remote procedure call interfaces for future API integration, enabling external systems to interact with the blockchain.

### 5.2 Crypto Module

The `crypto` module underpins the security of the system with the following responsibilities:

- **Hashing (`hashing.rs`)**: Implements robust cryptographic hash functions to ensure data integrity.
- **Key Management (`keys.rs`)**: Generates and securely manages cryptographic key pairs used by wallets and nodes.
- **Digital Signatures (`signatures.rs`)**: Facilitates the signing of transactions and the verification of those signatures, ensuring that only authorized transactions are processed.

### 5.3 Wallet Module

The `wallet` module is the user-facing component, enabling interactions with the blockchain:

- **Key Storage and Management**: Each wallet maintains a unique key pair for secure transaction signing.
- **Transaction Construction**: Wallets select appropriate UTXOs, sign transactions with the private key, and then broadcast them for validation.
- **Balance Tracking**: By monitoring UTXO changes, wallets can accurately report the user’s available funds.

## 6. Transaction Lifecycle

The journey of a transaction through the system can be broken down into several steps:

1. **Initiation**: A user starts a transaction via the wallet interface, specifying the recipient and amount.
2. **Signature Process**: The wallet uses the user’s private key to sign the transaction, ensuring its authenticity.
3. **Broadcasting**: The signed transaction is sent to the network, where validator nodes perform integrity checks.
4. **Mempool Inclusion**: Valid transactions are added to the mempool, a temporary holding area before block formation.
5. **Block Formation and Mining**: Miner nodes select transactions from the mempool, compile them into a block, and solve the PoW challenge.
6. **Validation and Chain Update**: Once a block is mined, it’s broadcast to the network. Validator nodes verify its correctness before adding it to their copy of the blockchain.

## 7. Security Considerations

Security is paramount in this implementation:

- **Digital Signatures**: Ensure that transactions are authorized by the rightful owners without exposing private keys.
- **Proof of Work**: Adds computational difficulty to block creation, deterring malicious actors from attempting to tamper with the blockchain.
- **UTXO Validation**: Prevents double-spending by ensuring that each transaction only uses available unspent outputs.
- **Consensus Mechanisms**: Aid in resolving conflicts and maintaining a unified ledger across all network nodes.

## 8. Future Work and Roadmap

As the project evolves, several enhancements are planned:

- **Transition to P2P Networking**: Move from simulation-based node interaction to a fully distributed peer-to-peer network.
- **Advanced Consensus Protocols**: Optimize and secure the consensus mechanism further with additional safeguards.
- **Web Interface Development**: Implement a real-time web interface for user account management, transaction visualization, and blockchain monitoring.
- **Transaction Fee and UTXO Optimization**: Introduce transaction fees to incentivize mining and improve UTXO selection strategies.
- **Robust Error Handling**: Strengthen error-handling across modules to ensure resilience in adverse conditions.

## 9. Conclusion

This Rust-based cryptocurrency implementation aims to provide a comprehensive, modular, and secure foundation for a Bitcoin-like system. By dividing responsibilities among well-defined modules—core blockchain logic, cryptographic operations, and wallet management—the project aims to demonstrate an in-depth understanding of decentralized systems. As development continues, the focus will be on enhancing networking capabilities, refining security measures, and expanding the user interface, paving the way for real-world applications in decentralized finance and beyond.

---
