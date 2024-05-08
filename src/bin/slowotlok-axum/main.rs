use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use slowotlok_backend::repository::RepositoryTrait;
use slowotlok_backend::repository_sqlite::RepositorySqlite;
use slowotlok_backend::{
    dtos::{AddCardDTO, CardDTO, ImportCardsResponseDTO},
    models::NewCard,
};

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
    Json(payload): Json<AddCardDTO>,
) -> Response {
    println!("Adding card...");
    let mut r = repo.lock().unwrap();
    let result = r.insert(NewCard::new(payload.src.clone(), payload.tr.clone()));
    drop(r);
    match result {
        Ok(entity) => Json(CardDTO::from_entity(&entity)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error).into_response(),
    }
}

async fn rm_card(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Path(id): Path<i32>,
) -> Response {
    let mut r = repo.lock().unwrap();
    match r.delete(id) {
        Ok(id) => Response::new("".into()),
        Err(error) => Response::new(error.into()),
    }
}

async fn get_cards(State(repo): State<Arc<Mutex<impl RepositoryTrait>>>) -> Response {
    let mut r = repo.lock().unwrap();
    let out: Vec<CardDTO> = r.all().iter().map(|x| CardDTO::from_entity(x)).collect();
    Json(out).into_response()
}
async fn update_card(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Json(payload): Json<CardDTO>,
) -> Response {
    let c = CardDTO::to_entity(payload);
    let mut r = repo.lock().unwrap();
    match r.update(&c) {
        slowotlok_backend::simple_repository::RepositorySimpleResult::OK => {
            let out = CardDTO::from_entity(&c);
            return Json(out).into_response();
        }
        slowotlok_backend::simple_repository::RepositorySimpleResult::Failed(error_text) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error_text).into_response();
        }
    }
}

async fn import_cards(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Json(payload): Json<Vec<AddCardDTO>>,
) -> Response {
    let mut added = vec![];
    let mut errors = vec![];
    let mut r = repo.lock().unwrap();
    for card in payload.iter() {
        match r.insert(NewCard::new(card.src.clone(), card.tr.clone())) {
            // Ok(entity) => added.push(CardDTO::from_entity(&entity)),
            Ok(entity) => added.push(entity.into()),
            Err(error) => errors.push(error),
        }
    }
    Json(ImportCardsResponseDTO::new(added, errors)).into_response()
}

async fn get_cards_random(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Path(count): Path<usize>,
) -> Response {
    let r = repo.lock().unwrap();
    let out: Vec<_> = r
        .random(count)
        .iter()
        .map(|x| CardDTO::from_entity(x))
        .collect();
    Json(out).into_response()
}

// async fn get_cards_random_lowest(Path(count): Path<usize>) -> Response {
//     (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
// }
