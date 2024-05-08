use crate::{models::{CardEntity, NewCard}, simple_repository::RepositorySimpleResult};

pub trait RepositoryTrait {
    fn random(&self, count: usize) -> Vec<CardEntity>;
    fn count(&self) -> usize;
    fn all(&mut self) -> Vec<CardEntity>;
    fn insert(&mut self, card: NewCard) -> Result<CardEntity, String>;
    fn insert_list(&mut self, cards: &mut Vec<CardEntity>) -> RepositorySimpleResult;
    fn update(&mut self, card: &CardEntity) -> RepositorySimpleResult;
    fn delete(&mut self, id: i32) -> Result<i32, String>;
}