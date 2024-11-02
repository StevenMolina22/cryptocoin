use crate::crypto::generate_key_pair;

#[derive(Debug, Hash, Clone)]
pub struct Date {
    day: u8,
    month: u8,
    year: u32,
}

#[derive(Debug, Hash, Clone)]
pub struct Signature {
    hash: Vec<u8>,
    sk: [u8; 32],
}

#[derive(Debug, Hash, Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub hashed_password: String,
    pub pk: [u8; 32],
    pub sk: [u8; 32],
}

impl Signature {
    pub fn new(message: String, sk: [u8; 32]) -> Signature {
        Signature {
            hash: vec![], // TODO: hash the message
            sk,
        }
    }
}

impl Date {
    pub fn new(day: u8, month: u8, year: u32) -> Date {
        Date { day, month, year }
    }
}

impl User {
    pub fn new(username: &str) -> User {
        let keypair = generate_key_pair();
        User {
            id: 0, // TODO: generate a unique id
            name: username.to_string(),
            hashed_password: String::from(""), // TODO: hash the password
            pk: keypair.public.to_bytes(),
            sk: keypair.secret.to_bytes(),
        }
    }
}
