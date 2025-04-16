# Bitcoin-like Cryptocurrency Implementation in Rust

## Table of Contents

1. [Introduction](#1-introduction)
2. [Project Goals](#2-project-goals)
3. [Architecture and Design Principles](#3-architecture-and-design-principles)
4. [System Architecture](#4-system-architecture)
5. [File Structure Overview](#5-file-structure-overview)
6. [Module Details](#6-module-details)
   - [6.1 Core Module](#61-core-module)
     - [6.1.1 Block Submodule](#611-block-submodule-coreblock)
     - [6.1.2 Chain Submodule](#612-chain-submodule-corechain)
     - [6.1.3 Network Submodule](#613-network-submodule-corenetwork)
     - [6.1.4 Transaction Submodule](#614-transaction-submodule-coretransaction)
     - [6.1.5 CLI and RCP Submodules](#615-cli-and-rcp-submodules)
   - [6.2 Crypto Module](#62-crypto-module)
   - [6.3 Wallet Module](#63-wallet-module)
7. [Transaction Lifecycle](#7-transaction-lifecycle)
8. [Mining Process](#8-mining-process)
9. [Node Interactions](#9-node-interactions)
10. [Security Considerations](#10-security-considerations)
11. [Future Work and Roadmap](#11-future-work-and-roadmap)
12. [Conclusion](#12-conclusion)

## 1. Introduction

This project is a simplified Bitcoin-like cryptocurrency implemented in Rust. It's designed as both an educational and experimental platform, demonstrating core blockchain principles such as decentralized ledger management, transaction validation, Proof of Work (PoW) consensus, and secure cryptographic operations. Though still in early development, it provides a solid foundation for understanding and extending blockchain technology.

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

## 4. System Architecture

### High-Level Component Architecture

```mermaid
flowchart TB
    subgraph User["User Interaction"]
        Wallet["Wallet Module"]
    end

    subgraph Core["Core Components"]
        Block["Block Module"]
        Chain["Chain Module"]
        Transaction["Transaction Module"]
        Network["Network Module"]
        CLI["CLI Module"]
        RCP["RCP Module"]
    end

    subgraph Security["Security Layer"]
        Crypto["Crypto Module"]
    end

    Wallet -->|Uses| Transaction
    Wallet -->|References| Chain
    Wallet -->|Uses| Crypto

    Block -->|Uses| Crypto
    Block -->|Contains| Transaction

    Chain -->|Manages| Block
    Chain -->|Validates| Transaction
    Chain -->|Maintains| UTXO["UTXO Set"]

    Network -->|Propagates| Block
    Network -->|Propagates| Transaction
    Network -->|Syncs| Chain

    CLI -->|Interfaces with| Chain
    CLI -->|Interfaces with| Wallet

    RCP -->|Exposes| Chain
    RCP -->|Exposes| Wallet
```

## 5. File Structure Overview

The project is organized as follows:

```
src
├── core
│  ├── block
│  │  ├── accessors.rs         # Functions to retrieve block properties
│  │  ├── mod.rs               # Block module interface and definitions
│  │  ├── pow.rs               # Proof of Work implementation (mining)
│  │  └── transactions.rs      # Block-specific transaction handling
│  ├── chain
│  │  ├── chain.rs             # Blockchain management and ledger state
│  │  ├── consensus.rs         # Consensus rules and conflict resolution
│  │  ├── mempool.rs           # Management of pending transactions
│  │  ├── mod.rs               # Chain module interface and BlockChain definition
│  │  └── transactions.rs      # Transaction integration within the chain
│  ├── cli
│  │  └── mod.rs               # Command-line interface for direct blockchain interaction
│  ├── mod.rs                  # Entry point for core functionalities
│  ├── network
│  │  ├── block.rs             # Block propagation and network validation logic
│  │  ├── blockchain.rs        # Synchronization of blockchain states across nodes
│  │  ├── mod.rs               # Network module interface and Node definition
│  │  └── transaction.rs       # Network transaction broadcasting and validation
│  ├── rcp
│  │  └── mod.rs               # Remote procedure call (API) interfaces
│  └── transaction
│     ├── mod.rs               # Transaction module interface and definitions
│     ├── transactions.rs      # Creation, signing, and validation of transactions
│     └── utxo.rs              # Unspent Transaction Output (UTXO) management
├── crypto
│  ├── hashing.rs              # Cryptographic hashing functions (SHA3-256)
│  ├── keys.rs                 # Key generation and key pair management
│  ├── mod.rs                  # Crypto module entry point
│  └── signatures.rs           # Digital signature creation and verification routines
├── main.rs                    # Entry point with simulation and testing
└── wallet
   ├── mod.rs                  # Wallet interface; handles key management
   └── transactions.rs         # Wallet-specific transaction construction
```

## 6. Module Details

### 6.1 Core Module

The `core` module is the heart of the blockchain logic, encompassing several submodules:

### 6.1.1 Block Submodule (`core/block`)

- **Responsibilities**:
  - Construct blocks containing multiple transactions.
  - Mine blocks by solving the PoW challenge defined in `pow.rs`.
  - Provide accessor functions for querying block data.
- **Key Features**:
  - **Proof of Work (PoW)**: Implemented in `pow.rs`, it ensures that new blocks satisfy a predefined difficulty by finding a nonce that produces a hash with required leading zeros.
  - **Block Structure**: Contains ID, previous hash, timestamp, nonce, transactions list, and own hash.
  - **Block Creation**: `new_template` method creates a block ready for mining, including a coinbase transaction to reward miners.

```mermaid
classDiagram
    class Block {
        +String id
        +String previous_hash
        +uint64 timestamp
        +uint64 nonce
        +Vec~Transaction~ transactions
        -Option~String~ hash
        +new_template(previous_hash: &str, miner: &str, reward: usize, transactions: Vec~Transaction~): Block
        +mine(difficulty: usize): void
        +is_valid(difficulty: usize): Result~(), SignatureError~
        +hash(): Option~String~
    }

    class Transaction {
        +Vec~TxInput~ inputs
        +Vec~TxOutput~ outputs
        +String id
    }

    Block *-- Transaction : contains
```

### 6.1.2 Chain Submodule (`core/chain`)

- **Responsibilities**:
  - Manage the blockchain ledger, ensuring proper block order and integrity.
  - Maintain the UTXO set to track unspent transaction outputs.
  - Handle transaction validation and inclusion in blocks.
- **Key Components**:
  - **BlockChain Structure**: Manages blocks, mempool, UTXOs, difficulty, and reward.
  - **UTXO Management**: Methods for adding and removing UTXOs when processing blocks.
  - **Transaction Processing**: `include_transaction` method validates and adds transactions to the mempool.
  - **Block Processing**: `update_from` method adds a block to the chain and updates the UTXO set accordingly.

```mermaid
classDiagram
    class BlockChain {
        +Vec~Block~ blocks
        +Vec~Transaction~ mempool
        +HashMap~(String, usize), UTXO~ utxos
        +usize difficulty
        +usize reward
        +new(): BlockChain
        +last_hash(): String
        +update_from(block: Block): void
        +remove_input_utxos(block: &Block): void
        +create_output_utxos(block: &Block): void
        +utxos_of(addr: &str): Vec~UTXO~
        +mine_funds_for(miner: &str): void
    }

    class Block {
        +String id
        +String previous_hash
        +uint64 timestamp
        +uint64 nonce
        +Vec~Transaction~ transactions
        -Option~String~ hash
    }

    class UTXO {
        +String tx_id
        +usize index
        +usize amount
        +String recipient
        +new(txid: &str, idx: usize, amount: usize, recipient: &str): UTXO
    }

    class Transaction {
        +Vec~TxInput~ inputs
        +Vec~TxOutput~ outputs
        +String id
    }

    BlockChain *-- Block : manages
    BlockChain *-- Transaction : mempool
    BlockChain *-- UTXO : tracks
```

### 6.1.3 Network Submodule (`core/network`)

- **Responsibilities**:
  - Simulate decentralized network behavior for testing and development.
  - Define Node structure that manages blockchain operations in a network context.
- **Key Components**:
  - **Node Implementation**: Contains methods for validating transactions, mining new blocks, and validating blocks.
  - **Simulation Methods**: `broadcast` methods that simulate network propagation of blocks and transactions.

```mermaid
classDiagram
    class Node {
        +Wallet wallet
        +new(wallet: Wallet): Node
        +validate_tx(tx: Transaction, pk: &PublicKey): Result~Transaction, SignatureError~
        +mine(transactions: Vec~Transaction~, pk: &PublicKey): Result~Block, SignatureError~
        +validate_block(block: Block): Result~Block, SignatureError~
    }

    class Block {
        +listen_from_network(): void
        +broadcast(): Block
    }

    class Transaction {
        +listen_from_network(): Vec~Transaction~
        +broadcast(): Transaction
    }

    class BlockChain {
        +retrieve_from_network(): BlockChain
        +broadcast(): BlockChain
    }

    class Wallet {
    }

    Node *-- Wallet : contains
    Node ..> Block : mines/validates
    Node ..> Transaction : validates
    Wallet *-- BlockChain : references
```

### 6.1.4 Transaction Submodule (`core/transaction`)

- **Responsibilities**:
  - Define the structure and behavior of transactions in the blockchain.
  - Manage transaction creation, validation, and UTXO handling.
- **Key Components**:
  - **Transaction Structure**: Contains ID, inputs, and outputs.
  - **UTXO Implementation**: TxInput, TxOutput, and UTXO structures for the UTXO model.
  - **Transaction Creation**: Methods for creating both regular and coinbase transactions.
  - **Validation**: `is_valid` method to verify transaction signatures.

```mermaid
classDiagram
    class Transaction {
        +Vec~TxInput~ inputs
        +Vec~TxOutput~ outputs
        +String id
        +new(amount: usize, sender: &str, recipient: &str, utxos: &UTXOPool, keypair: &mut Keypair): Result~Transaction, TransactionError~
        +new_coinbase(recipient: &str, reward: usize): Transaction
        +is_valid(pk: &PublicKey): Result~(), SignatureError~
    }

    class TxInput {
        +String tx_id
        +usize index
        -Signature signature
        +new(tx_id: &str, index: usize, keypair: &mut Keypair): TxInput
        +is_valid(pk: &PublicKey): Result~(), SignatureError~
    }

    class TxOutput {
        +usize amount
        +String recipient
        +new(recipient: &str, amount: usize): TxOutput
    }

    class UTXO {
        +String tx_id
        +usize index
        +usize amount
        +String recipient
        +new(txid: &str, idx: usize, amount: usize, recipient: &str): UTXO
    }

    Transaction *-- TxInput : contains
    Transaction *-- TxOutput : contains
    TxInput ..> UTXO : references
```

### 6.1.5 CLI and RCP Submodules

- **CLI**: Currently a placeholder module for future command-line interface implementation.
- **RCP**: Placeholder for future remote procedure call interfaces.

### 6.2 Crypto Module

The `crypto` module provides cryptographic functionality:

- **Hashing**: Implements SHA3-256 hashing for blocks and transactions.
- **Key Management**: Generates ed25519 key pairs for wallet creation.
- **Signatures**: Framework for digital signature operations.

```mermaid
classDiagram
    class Crypto {
        +hash_block(block: &Block): String
        +generate_key_pair(): Keypair
    }
```

### 6.3 Wallet Module

The `wallet` module enables user interaction with the blockchain:

- **Wallet Structure**: Contains blockchain reference, address (public key representation), and keypair.
- **Transaction Creation**: `transfer` method creates and signs new transactions.
- **Initialization**: `new` method creates a wallet with a fresh keypair.

```mermaid
classDiagram
    class Wallet {
        +BlockChain blockchain
        +String address
        +Keypair keypair
        +new(blockchain: BlockChain): Wallet
        +transfer(receiver: &str, amount: usize): Result~(), SignatureError~
    }

    class BlockChain {
    }

    Wallet *-- BlockChain : references
```

## 7. Transaction Lifecycle

The journey of a transaction through the system:

```mermaid
sequenceDiagram
    participant User
    participant Wallet
    participant Node1 as Validator Node
    participant Node2 as Miner Node
    participant Node3 as Network Node

    User->>Wallet: Request transfer(recipient, amount)
    activate Wallet

    Note over Wallet: 1. Select UTXOs
    Note over Wallet: 2. Create transaction
    Note over Wallet: 3. Sign inputs
    Note over Wallet: 4. Validate signature

    Wallet-->>Node1: Broadcast transaction
    deactivate Wallet

    activate Node1
    Note over Node1: 5. Validate transaction
    Note over Node1: 6. Add to mempool
    Node1-->>Node2: Propagate transaction
    deactivate Node1

    activate Node2
    Note over Node2: 7. Collect transactions
    Note over Node2: 8. Create block template
    Note over Node2: 9. Mine block (PoW)

    Node2-->>Node3: Broadcast mined block
    deactivate Node2

    activate Node3
    Note over Node3: 10. Validate block
    Note over Node3: 11. Add to blockchain
    Note over Node3: 12. Update UTXO set
    deactivate Node3
```

## 8. Mining Process

```mermaid
flowchart TD
    A[Start Mining] --> B{Empty Mempool?}
    B -->|Yes| C[Create Block with Only Coinbase TX]
    B -->|No| D[Select Transactions from Mempool]
    D --> E[Create Block Template with Transactions]
    C --> F[Set Initial Nonce to 0]
    E --> F
    F --> G[Compute Block Hash]
    G --> H{Hash Meets Difficulty?}
    H -->|No| I[Increment Nonce]
    I --> G
    H -->|Yes| J[Block Successfully Mined]
    J --> K[Broadcast Block to Network]
    K --> L[Update Local Blockchain]
    L --> M[Remove Mined Transactions from Mempool]
    M --> N[Create Output UTXOs]
    N --> O[End Mining Process]
```

## 9. Node Interactions

```mermaid
stateDiagram-v2
    [*] --> Idle

    Idle --> ReceivingTransaction: New Transaction Received
    Idle --> ReceivingBlock: New Block Received
    Idle --> Mining: Start Mining

    ReceivingTransaction --> ValidatingTransaction: Process Transaction
    ValidatingTransaction --> AddingToMempool: Valid Transaction
    ValidatingTransaction --> RejectingTransaction: Invalid Transaction

    AddingToMempool --> Idle: Transaction Added
    RejectingTransaction --> Idle: Transaction Rejected

    ReceivingBlock --> ValidatingBlock: Process Block
    ValidatingBlock --> UpdatingChain: Valid Block
    ValidatingBlock --> RejectingBlock: Invalid Block

    UpdatingChain --> RemovingFromMempool: Update Blockchain
    RemovingFromMempool --> UpdatingUTXOSet: Remove Transactions from Mempool
    UpdatingUTXOSet --> Idle: Update Complete

    RejectingBlock --> Idle: Block Rejected

    Mining --> CreatingBlock: Select Transactions
    CreatingBlock --> MiningBlock: Create Block Template
    MiningBlock --> BroadcastingBlock: Found Valid Nonce
    BroadcastingBlock --> Idle: Block Broadcasted

    Mining --> Idle: Interrupt Mining
```

## 10. Security Considerations

Security measures in the implementation:

- **Digital Signatures**: Ed25519 signatures ensure transaction authenticity.
- **Proof of Work**: Mining difficulty prevents easy chain manipulation.
- **UTXO Validation**: Prevents double-spending by tracking spent outputs.
- **Transaction Verification**: Multiple validation points ensure transaction integrity.

```mermaid
flowchart LR
    subgraph Authentication
        DS[Digital Signatures]
        KM[Key Management]
    end

    subgraph Consensus
        POW[Proof of Work]
        DIF[Dynamic Difficulty]
    end

    subgraph DataIntegrity
        HASH[SHA3-256 Hashing]
        CHAIN[Chain Validation]
    end

    subgraph DoubleSpendPrevention
        UTXO[UTXO Tracking]
        MV[Multi-point Validation]
    end

    DS --> TXV[Transaction Validation]
    KM --> DS

    POW --> BV[Block Validation]
    DIF --> POW

    HASH --> POW
    HASH --> BV
    CHAIN --> BV

    UTXO --> TXV
    MV --> TXV
    MV --> BV
```

## 11. Future Work and Roadmap

Planned enhancements:

```mermaid
gantt
    title Development Roadmap
    dateFormat  YYYY-MM-DD

    section Core Stability
    Complete Transaction Validation Implementation :a1, 2025-04-15, 30d
    Improve UTXO Management                        :a2, after a1, 45d
    Enhance Error Handling                         :a3, after a2, 30d
    Add Comprehensive Test Coverage                :a4, 2025-04-20, 60d

    section Network Implementation
    Implement P2P Network Layer                    :b1, after a2, 60d
    Develop Node Discovery Mechanism               :b2, after b1, 30d
    Add Blockchain Synchronization                 :b3, after b2, 45d
    Implement Fork Resolution                      :b4, after b3, 45d

    section User Experience
    Develop CLI Interface                          :c1, after a3, 30d
    Implement RPC API                              :c2, after c1, 45d
    Create Web Interface                           :c3, after c2, 60d
    Add Wallet Backup & Recovery                   :c4, after c3, 30d

    section Advanced Features
    Implement Transaction Fees                     :d1, after b4, 30d
    Add Simple Smart Contracts                     :d2, after d1, 60d
    Enhance Security Features                      :d3, after d2, 45d
    Research Advanced Consensus                    :milestone, after d3, 0d
```

## 12. Conclusion

This Rust cryptocurrency implementation provides a solid foundation for understanding blockchain technology through a hands-on approach. The modular architecture allows for incremental improvements and learning. While primarily educational in nature, the project demonstrates the core principles that underpin real-world cryptocurrencies: secure transactions, consensus through proof-of-work, and a transparent, immutable ledger.

As development continues, the focus will be on enhancing security, networking capabilities, and usability, while maintaining the educational value of the codebase.

---
