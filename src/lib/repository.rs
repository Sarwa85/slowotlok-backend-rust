use crate::{card::Card, simple_repository::RepositorySimpleResult};

pub trait RepositoryTrait {
    fn random(&self, count: usize) -> Vec<Card>;
    fn count(&self) -> usize;
    fn all(&self) -> Vec<Card>;
    fn insert(&mut self, card: &mut Card) -> RepositorySimpleResult;
    fn insert_list(&mut self, cards: &mut Vec<Card>) -> RepositorySimpleResult;
    fn update(&mut self, card: &Card) -> RepositorySimpleResult;
    fn delete(&mut self, card: &Card) -> RepositorySimpleResult;
    fn delete_by_id(&mut self, id: i64) -> RepositorySimpleResult;
}