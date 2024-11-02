**`FLOW of APP`**

A user has an amount of money already in its account, then he transfers that money to another account, this is done by in the ledger he writes a description of the transaction with his signature, the receiver is sure that the signature is valid so he accepts the transaction as valid.

A transaction is a record about the sender, receiver, amount and extra details, the ledger in this case is a record of all transactions, the signature is something the uniquely identifies the sender and that can be easily checked for validity.

---

Concepts

- User
- Accounts → Wallet / App
- Transactions → Common
- Ledger → Blockchain
- Signature

---

Each transaction made in any wallet must update the block chain

---

**`MODULES AND SYSTEM`**

The user interface with the system will be wallet.

The currency itself will be a simple ledger with signatures capabilities.

The distribution system of the currency will be the blockchain.

**`Wallet module:`**

- Manages user interactions
    - Transactions
    - Balance Check…
- Keeps track of users balances
- Stores keys and signatures for transactions

**`Blockchain module:`**

- Handles security
- Handles consensus
- Handles transaction/block history

**`Crypto module:`**

- Cryptographic logic
- Hashing, signatures and keys
- Tokens

---

- RECOMMENDATIONS IMPLEMENTATION

    Dividing project into distinct modules—**wallet**, **blockchain**, and **currency**—

    ### 1. **Modular Design Benefits**

    - **Specialization**: Each module can focus on specific functionalities, allowing for specialized development and optimization. This separation can enhance the quality and performance of each component.
    - **Scalability**: Modular designs facilitate easier updates and scalability. You can upgrade or replace one module without affecting the others, making it simpler to adapt to new technologies or user needs.
    - **Collaboration**: Different teams can work concurrently on separate modules, speeding up the development process and allowing for parallel workflows.

    ### 2. **Wallet Module**

    - **User Interface**: Focus on creating a user-friendly interface that allows users to manage their assets easily. Consider features like transaction history, balance display, and security settings.
    - **Security Features**: Implement robust security measures such as encryption, two-factor authentication, and backup options to protect user funds.
    - **Integration**: Ensure compatibility with various blockchain networks if you plan to support multiple currencies.

    ### 3. **Blockchain Module**

    - **Consensus Mechanism**: Choose an appropriate consensus mechanism (e.g., Proof of Work, Proof of Stake) based on your project's goals and resource availability[2].
    - **Transaction Processing**: Design the architecture for how transactions will be verified and added to the blockchain, including node communication protocols[5].
    - **Smart Contracts**: If applicable, implement smart contract functionality to automate processes within your ecosystem.

    ### 4. **Currency Module**

    - **Token-economics**: Define the economic model of your currency, including total supply, distribution methods, and incentives for users[2].
    - **Utility**: Clearly outline the use cases for your currency within your ecosystem to encourage adoption and usage.
    - **Regulatory Compliance**: Consider legal implications related to creating a new currency, including adherence to local regulations regarding cryptocurrencies.

    ### 5. **Testing and Quality Assurance**

    - **Unit Testing**: Conduct thorough testing of each module independently to ensure functionality before integration.
    - **Integration Testing**: After individual modules are tested, perform integration testing to ensure they work seamlessly together.

    ### 6. **Documentation and Community Engagement**

    - **Documentation**: Maintain clear documentation for each module that outlines functionality, APIs, and integration points.
    - **Community Feedback**: Engage with potential users early in the development process to gather feedback on usability and features.
- Graphics

    ```mermaid
    graph TD;
        A[Crypto Currency Back-End] --> B[Calculations]
        A --> C[Hashing]
        A --> D[Security]
        A --> E[Wallet Back-End]
        A --> F[Interface for Interaction]

        F --> G[User Inputs]
        F --> H[Network Inputs]
        F --> I[Front-End Inputs]
    ```

    ```mermaid
    graph TD
      A[Input Handler] --> B[Wallet]
      B --> C[BlockChain]
      C --> D[Currency]

    ```


**`IMPLEMENTATION`**

**Working with the interfaces:**

The crypto currency will have a complete back-end that takes cares of everything related to calculations, hashing and security, as well as a wallet back-end, it will also have an interface for interacting with the outside world, this for the reason to accept different types of inputs, let it be user, network, front-end, etc.

Current idea:

The input will come from a website, where there will be the option to sign up into an account, this already will give the possibility to start to make transactions, the wallet will be a digital one, there will be too a back-end that will hold users information, keys, balance amounts, etc, as well as the history of transactions of all the network, (should be able to be decentralized eventually)

Of course the front-end for the wallet will use the back-end which in turn uses the wallet interface, which will have methods for interacting with users money and to start making transactions, the front-end and back-end will have absolute no interaction with the implementation from crypto-blockchain.

The wallet will interact directly with the blockchain, that is the one that keeps track of the history of transactions as well as most of the security part of it.

- **Web-App Interface:**

    The web app interface

- **Wallet interface:**
    - Create Wallet

        Wallet will be object in which transfer are directly made

    - Get balance
    - Send funds
    - Receive funds

    ---

    - Get transactions history
    - Verify transaction
    - Lock wallet
    - Unlock wallet
- **Blockchain Interface:**

    The blockchains main goal is to handle security and everything related to that

    - Create blog
    - Add Blog to chain
    - Get Blog by Height
    - Get Latest Block
    - Verify Block
    - Get chain length
    - Mine Block
    - Check Consensus
    - Get transaction history

    ---

    - Replace chain
- **Crypto Interface**

    The crypto interface will be characterize for the ability to do this things:

    ---

    - Create signature
    - Verify signature
    - Generate PK SK pair
