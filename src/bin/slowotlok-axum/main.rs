use std::{
    borrow::BorrowMut,
    string,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Error, Json, Router,
};
use serde::{Deserialize, Serialize};
use slowotlok_backend_rust::{card::Card, repo::Repository};

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    let mut repo = Arc::new(RwLock::new(Repository::new()));

    let app = Router::new()
        .route("/card", post(add_card))
        .with_state(repo);
    // .route("/card", get(get_cards))
    // // .put(update_card))
    // .route("/card/import", post(import_cards))
    // .route("/card/random/:count", get(get_cards_random))
    // .route("/card/random_lowest/:count", get(get_cards_random_lowest));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_error(error: Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", error),
    )
}

async fn add_card(
    State(repo): State<Arc<RwLock<Repository>>>,
    Json(payload): Json<AddCardDTO>,
) -> Response {
    let mut c = Card::new(payload.src, payload.tr);
    match repo.write().unwrap().insert(&mut c) {
        slowotlok_backend_rust::repo::RepositorySimpleResult::OK => {
            return Json(c).into_response();
        }
        slowotlok_backend_rust::repo::RepositorySimpleResult::Failed(error_text) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error_text).into_response();
        }
    }
}

async fn get_cards() -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
}
async fn update_card(Json(payload): Json<CardDTO>) -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
}

async fn import_cards() -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
}

async fn get_cards_random(Path(count): Path<usize>) -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
}

async fn get_cards_random_lowest(Path(count): Path<usize>) -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
}

#[derive(Deserialize)]
struct AddCardDTO {
    src: String,
    tr: String,
}

#[derive(Deserialize)]
struct CardDTO {
    id: usize,
    src: String,
    tr: String,
    good: usize,
    bad: usize,
}

#[derive(Deserialize)]
struct ErrorDTO {
    message: String,
}
