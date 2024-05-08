use diesel::prelude::*;

use crate::dtos::{AddCardDTO, CardDTO};

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::card)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
    pub id: i32,
    pub src: String,
    pub tr: String,
    pub good: i32,
    pub bad: i32,
}

impl From<CardDTO> for Card {
    fn from(value: CardDTO) -> Self {
        Card{id: value.id, src: value.src, tr: value.tr, good: value.good, bad: value.bad}
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::card)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCard {
    pub src: String,
    pub tr: String,
    good: i32,
    bad: i32
}

impl NewCard {
    pub fn new(src: String, tr: String) -> Self{
        NewCard{src, tr, good: 0, bad: 0}
    }
}

impl From<AddCardDTO> for NewCard {
    fn from(value: AddCardDTO) -> Self {
        NewCard::new(value.src, value.tr)
    }
}