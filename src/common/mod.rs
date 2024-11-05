#[derive(Debug, Hash, Clone, serde::Serialize)]
pub struct Date {
    day: u8,
    month: u8,
    year: u32,
}

#[derive(Debug, Hash, Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub hashed_password: String,
    pub pk: [u8; 32],
    pub sk: [u8; 32],
}

impl Date {
    pub fn new(day: u8, month: u8, year: u32) -> Date {
        Date { day, month, year }
    }
}
