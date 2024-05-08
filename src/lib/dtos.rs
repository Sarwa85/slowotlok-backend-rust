use serde::{Deserialize, Serialize};

use crate::models::Card;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardDTO {
    pub id: i32,
    pub src: String,
    pub tr: String,
    pub good: i32,
    pub bad: i32,
}

impl From<Card> for CardDTO {
    fn from(value: Card) -> Self {
        CardDTO {
            id: value.id,
            src: value.src,
            tr: value.tr,
            good: value.good,
            bad: value.bad,
        }
    }
}

impl CardDTO {
    pub fn new(src: String, tr: String) -> Self {
        Self {
            id: -1,
            src,
            tr,
            good: 0,
            bad: 0,
        }
    }
}

#[derive(Deserialize)]
pub struct AddCardDTO {
    pub src: String,
    pub tr: String,
}

// #[derive(Deserialize)]
// struct ErrorDTO {
//     message: String,
// }

#[derive(Serialize)]
pub struct ImportCardsResponseDTO {
    added: Vec<CardDTO>,
    errors: Vec<String>,
}

impl ImportCardsResponseDTO {
    pub fn new(added: Vec<CardDTO>, errors: Vec<String>) -> Self {
        ImportCardsResponseDTO{added, errors}
    }
}
