use crate::models::{Card, NewCard};
use std::error::Error;

pub trait RepositoryTrait {
    fn random(&self, count: usize) -> Vec<Card>;
    fn count(&self) -> usize;
    fn all(&mut self) -> Vec<Card>;
    fn insert(&mut self, card: NewCard) -> Result<Card, Box<dyn Error>>;
    fn insert_list(&mut self, cards: &mut Vec<NewCard>) -> Result<Vec<Card>, String>;
    fn update(&mut self, card: &Card) -> Result<Card, String>;
    fn delete(&mut self, id: i32) -> Result<i32, String>;
}
