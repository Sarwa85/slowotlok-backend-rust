// use std::sync::{Arc};

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use slowotlok_backend::dtos::{AddCardDTO, CardDTO, ImportCardsResponseDTO};
use slowotlok_backend::repository_sqlite::RepositorySqlite;
use slowotlok_backend::{models::NewCard, repository::RepositoryTrait};

mod logic;

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // let mut repo: Arc<RwLock<dyn RepositoryTrait + Sync + Send>> = Arc::new(RwLock::new(SimpleRepository::new()));
    let repo = Arc::new(Mutex::new(RepositorySqlite::new()));

    let app = Router::new()
        .route("/card", post(add_card).get(get_cards).patch(update_card))
        .route("/card/:id", delete(rm_card))
        .route("/card/random/:count", get(get_cards_random))
        .route("/card/import", post(import_cards))
        .with_state(repo);
    // .route("/card/random_lowest/:count", get(get_cards_random_lowest));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn add_card(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Json(card): Json<AddCardDTO>,
) -> Response {
    match logic::add_card(repo, card).await {
        Ok(card) => Json(CardDTO::from(card)).into_response(),
        Err(error) => error.into_response(),
    }
}

async fn rm_card(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Path(id): Path<i32>,
) -> Response {
    let mut r = repo.lock().unwrap();
    match r.delete(id) {
        Ok(_id) => Response::new("".into()),
        Err(error) => Response::new(error.into()),
    }
}

async fn get_cards(State(repo): State<Arc<Mutex<impl RepositoryTrait>>>) -> Response {
    let mut repo_lock = repo.lock().unwrap();
    let result = repo_lock.all();
    drop(repo_lock);
    match result {
        Ok(cards) => Json(
            cards
                .into_iter()
                .map(|x| CardDTO::from(x))
                .collect::<Vec<CardDTO>>(),
        )
        .into_response(),
        Err(_) => todo!(),
    }
}
async fn update_card(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Json(payload): Json<CardDTO>,
) -> Response {
    let mut repo_lock = repo.lock().unwrap();
    let result = repo_lock.update(payload.into());
    drop(repo_lock);
    match result {
        Ok(c) => Json(CardDTO::from(c)).into_response(),
        Err(error_text) => {
            (StatusCode::INTERNAL_SERVER_ERROR, error_text.to_string()).into_response()
        }
    }
}

async fn import_cards(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Json(payload): Json<Vec<AddCardDTO>>,
) -> Response {
    let mut added = vec![];
    let mut errors = vec![];
    for card in payload.iter() {
        let mut repo_lock = repo.lock().unwrap();
        let result = repo_lock.insert(NewCard::new(card.src.clone(), card.tr.clone()));
        drop(repo_lock);
        match result {
            Ok(entity) => added.push(entity.into()),
            Err(error) => errors.push(error.to_string()),
        }
    }
    Json(ImportCardsResponseDTO::new(added, errors)).into_response()
}

async fn get_cards_random(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Path(count): Path<usize>,
) -> Response {
    let repo_lock = repo.lock().unwrap();
    let result = repo_lock.random(count);
    drop(repo_lock);
    match result {
        Ok(cards) => Json(
            cards
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<CardDTO>>(),
        )
        .into_response(),
        Err(error) => error.into_response(),
    }
}
