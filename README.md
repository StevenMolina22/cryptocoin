### **`OVERALL DESCRIPTION`**

This project is a Rust-based cryptocurrency implementation focused on secure, decentralized, and verifiable transactions managed on a blockchain using a Proof of Work (PoW) consensus model. Wallets are implemented to allow users to securely store their private (sk) and public (pk) keys, sign transactions, and interact with the blockchain. The project aims to provide both a robust backend handling cryptographic operations and a user interface via the wallet module to facilitate transactions and balance tracking.

At a high level, the network ensures ledger integrity and security through cryptographic mechanisms, such as hashing, digital signatures, and distributed consensus, providing both privacy and resilience against tampering.

Each transaction begins with a user’s wallet signing it, propagating through the network where other nodes validate, mine, and add it to the blockchain. Verification occurs at multiple stages:

1. **User Wallet**: Signs the transaction and ensures sufficient balance.
2. **Mining Node**: Receives the signed transaction, compiles it into a block, and competes in PoW to mine it.
3. **Consensus Verification**: All nodes receive and verify mined blocks, ensuring transaction authenticity before adding them to their copy of the blockchain.

---

### **`WALLET MODULE`**

The wallet is a critical part of the user interface, enabling users to manage their balances and execute transactions on the network. Wallets start with an initial balance, simplifying testing and early interactions on the network.

### Wallet Features:

- **Key Management**: Stores public/private key pairs (pk/sk) securely, used to sign transactions.
- **Balance Tracking**: Keeps account balances updated based on blockchain history.
- **Transaction Signing**: Ensures transactions are securely signed for propagation across the network.

---

### **`APPLICATION FLOW`**

1. **User Registration**: On signing up, users are assigned a unique wallet and an initial balance. The wallet is represented with a public address and private key.
2. **Transaction Creation**: Users initiate transfers by specifying the receiver's address and amount. This transaction is signed by the sender’s private key, allowing the receiver and network nodes to verify authenticity.
3. **Transaction Verification**:
    - The transaction includes the sender, receiver, amount, and signature.
    - The network validates the signature to confirm the sender's authenticity and verifies the wallet balance.
4. **Mining & Ledger Update**:
    - Once validated, the transaction is added to a block by a mining node.
    - The network’s PoW consensus model requires nodes to solve a computational problem, adding an element of security and finality to mined blocks.
    - Mined blocks, once verified by the network, are permanently added to the blockchain, solidifying transaction history.

---

### **`MODULES`**

### **Wallet Module**

- Manages user keys and interactions.
- Maintains balances and signing capabilities.

### **Blockchain Module**

- Ensures overall ledger security.
- Manages consensus, block verification, and transaction history.
- Implements PoW consensus to secure the network.

### **Block Module**

- Handles block creation, mining, and transaction storage.
- Executes PoW, making each block tamper-resistant.

### **Transaction Module**

- Manages transaction validation, propagation, and organization in blocks.

### **Crypto Module**

- Provides cryptographic utilities: hashing, digital signatures, and key management.
- Ensures that transactions and blocks remain secure, verifiable, and immutable.

---

### **`IMPLEMENTATION`**

The project’s backend handles the majority of logic, including cryptographic operations, transaction validation, and interaction with the blockchain. The wallet backend facilitates user interactions, balance tracking, and transaction initiation.

An eventual web interface will support user account management, enabling sign-ups and balance transfers with real-time transaction histories.

### **Intermodule Interactions**

- **Wallet Module to Blockchain**: Initiates transactions and verifies balances.
- **Blockchain to Crypto**: Uses cryptographic functions for hashing and signature verification.
- **All Modules to Transaction Module**: Central to maintaining an accurate, immutable record of transactions.

---

### **`TRANSACTION IMPLEMENTATION`**

Transactions occur exclusively between wallets, with each transaction linked to a block and logged in the blockchain.

1. **Linking Transactions to Blocks**:
    - If a block is already open, the transaction is appended to it.
    - If no active block is available, a new block is created, which initiates the mining process.
2. **Signature Mechanism**:
    - **Signing**: `(transaction, SK) -> Signature`
    - **Verification**: `(transaction, signature, PK) -> bool`
    - Every transaction signature verifies the sender’s identity without exposing their private key.

---

### **`DATA STRUCTURES`**

```rust
struct Wallet {
    address: String,
    balance: usize,
    pk: [u8; 32],
    sk: [u8; 32],
}

```

```rust
struct Blockchain {
    blocks: Vec<Block>,
}

```

```rust
struct Block {
    id: usize,
    previous_hash: String,
    timestamp: u64,
    nonce: u64,
    transactions: Vec<Transaction>,
}

```

```rust
struct Transaction {
    id: String,
    sender_addr: String,
    receiver_addr: String,
    signature: Option<Signature>,
    amount: usize,
    date: Date,
    transaction_type: TransactionType,
}

```

---

### **INTERFACES**

Using a modular project structure, each module will have well-defined interfaces to promote code reusability, encapsulation, and easier testing. Key files in the project include:

```
src
├── block
│  └── mod.rs
├── blockchain
│  ├── chain.rs
│  ├── concensus.rs
│  ├── mod.rs
│  └── transactions.rs
├── common
│  └── mod.rs
├── crypto
│  ├── keys.rs
│  ├── mod.rs
│  └── signatures.rs
├── main.rs
├── transaction
│  └── mod.rs
└── wallet
   └── mod.rs

```

Each module will implement an interface for its core functionalities, ensuring each component interacts effectively within the system. The `transaction` module is central to linking user actions to ledger records, while the `block` and `blockchain` modules manage PoW and consensus, maintaining the network’s integrity.
