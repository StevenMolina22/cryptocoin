#[derive(Debug, Hash, Clone, serde::Serialize, serde::Deserialize)]
pub struct Date {
    day: u8,
    month: u8,
    year: u32,
}

impl Date {
    pub fn new(day: u8, month: u8, year: u32) -> Date {
        Date { day, month, year }
    }
}
