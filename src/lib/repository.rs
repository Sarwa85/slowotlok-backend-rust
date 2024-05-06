use crate::{models::CardEntity, simple_repository::RepositorySimpleResult};

pub trait RepositoryTrait {
    fn random(&self, count: usize) -> Vec<CardEntity>;
    fn count(&self) -> usize;
    fn all(&mut self) -> Vec<CardEntity>;
    fn insert(&mut self, card: &mut CardEntity) -> RepositorySimpleResult;
    fn insert_list(&mut self, cards: &mut Vec<CardEntity>) -> RepositorySimpleResult;
    fn update(&mut self, card: &CardEntity) -> RepositorySimpleResult;
    fn delete(&mut self, card: &CardEntity) -> RepositorySimpleResult;
    fn delete_by_id(&mut self, id: i64) -> RepositorySimpleResult;
}