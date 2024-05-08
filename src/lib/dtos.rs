use serde::{Deserialize, Serialize};

use crate::models::CardEntity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardDTO {
    id: i32,
    src: String,
    tr: String,
    good: i32,
    bad: i32,
}

impl From<CardEntity> for CardDTO {
    fn from(value: CardEntity) -> Self {
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

    pub fn from_entity(entity: &CardEntity) -> CardDTO {
        CardDTO {
            id: entity.id,
            src: entity.src.clone(),
            tr: entity.tr.clone(),
            good: entity.good,
            bad: entity.bad,
        }
    }

    pub fn to_entity(dto: CardDTO) -> CardEntity {
        CardEntity {
            id: dto.id,
            src: dto.src,
            tr: dto.tr,
            good: dto.good,
            bad: dto.bad,
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
