use std::sync::{Arc, Mutex};

use slowotlok_backend::{apperror::AppError, dtos::{AddCardDTO, CardDTO}, models::NewCard, repository::RepositoryTrait};

pub async fn add_card(repo: Arc<Mutex<impl RepositoryTrait>>, new_card: AddCardDTO) -> anyhow::Result<CardDTO, AppError> {
    Ok(CardDTO::from(repo.lock().unwrap().insert(NewCard::new(new_card.src.clone(), new_card.tr.clone()))?))
}