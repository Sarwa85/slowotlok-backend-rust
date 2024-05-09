use crate::{apperror::AppError, models::{Card, NewCard}};

pub trait RepositoryTrait {
    fn random(&self, count: usize) -> anyhow::Result<Vec<Card>, AppError>;
    fn count(&self) -> usize;
    fn all(&mut self) -> anyhow::Result<Vec<Card>>;
    fn insert(&mut self, card: NewCard) -> anyhow::Result<Card>;
    fn update(&mut self, card: Card) -> anyhow::Result<Card>;
    fn delete(&mut self, id: i32) -> Result<i32, String>;
}
