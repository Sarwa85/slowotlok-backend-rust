use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: i64,
    pub source: String,
    pub translation: String,
    pub good: u32,
    pub bad: u32,
}

impl Card {
    pub fn new(source: String, translation: String) -> Self {
        Self {
            id: -1,
            source,
            translation,
            good: 0,
            bad: 0,
        }
    }
}
