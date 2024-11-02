use crate::blockchain::Blockchain;
use crate::crypto::{generate_key_pair, generate_signature, verify_signature};
use crate::transaction::{Transaction, TransactionType};

#[derive(Debug)]
pub struct Wallet {
    // user: User,
    pub address: String,
    pub balance: usize,
    // pub transactions: Vec<Transaction>,
    pub pk: [u8; 32],
    pub sk: [u8; 32],
}

impl Wallet {
    pub fn new(address: &str) -> Self {
        let keypair = generate_key_pair();
        Wallet {
            address: address.to_string(),
            balance: 0,
            pk: keypair.public.to_bytes(),
            sk: keypair.public.to_bytes(),
        }
    }

    /// ignore for now
    pub fn deposit(&mut self, amount: usize) -> Result<(), ()> {
        Ok(())
    }

    /// ignore for now
    pub fn withdraw(&mut self, amount: usize) -> Result<(), ()> {
        Ok(())
    }

    pub fn get_transactions(&self, blockchain: &Blockchain) -> Vec<Transaction> {
        blockchain.get_transaction_list()
    }

    pub fn get_balance(&self) -> usize {
        self.balance
    }

    pub fn transfer(
        &mut self,
        amount: usize,
        receiver_addr: &str,
        blockchain: &mut Blockchain,
    ) -> Result<(), ()> {
        let transaction = Transaction::new(
            String::from(""),
            amount,
            self.address.clone(),
            receiver_addr.to_string(),
            TransactionType::DebitCard,
        );
        let signature = generate_signature(&transaction, [0; 32]);
        println!("Signature: {:#?}", signature);
        let is_verified = verify_signature(&transaction, &signature, self.pk);
        if !is_verified {
            return Err(());
        }
        blockchain.add_transaction(transaction, self.pk)
    }
}

// value, err := mightErr()
// if err != nil {
//      ...
// }
//
// match mightErr() {
//      Ok(value) => { ... }
//      Err(err) => { ... }
// }
