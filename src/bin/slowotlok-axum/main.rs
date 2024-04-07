use std::{
    borrow::{Borrow, BorrowMut},
    string,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
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
        .route("/card", post(add_card).get(get_cards).patch(update_card))
        .route("/card/:id", delete(rm_card))
        .route("/card/random/:count", get(get_cards_random))
        .with_state(repo);
    // .route("/card/import", post(import_cards))
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

async fn rm_card(State(repo): State<Arc<RwLock<Repository>>>, Path(id): Path<i64>) -> Response {
    // let mut c = Card::new(payload.src, payload.tr);
    match repo.write().unwrap().delete_by_id(id) {
        slowotlok_backend_rust::repo::RepositorySimpleResult::OK => {
            return Response::new("".into());
        }
        slowotlok_backend_rust::repo::RepositorySimpleResult::Failed(error_text) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error_text).into_response();
        },
    }
}

async fn get_cards(State(repo): State<Arc<RwLock<Repository>>>) -> Response {
    let out = repo.read().unwrap().all();
    Json(out).into_response()
}
async fn update_card(
    State(repo): State<Arc<RwLock<Repository>>>,
    Json(payload): Json<CardDTO>,
) -> Response {
    let c = Card {
        id: payload.id,
        source: payload.src,
        translation: payload.tr,
        good: payload.good,
        bad: payload.bad,
    };
    match repo.write().unwrap().update(&c) {
        slowotlok_backend_rust::repo::RepositorySimpleResult::OK => {
            return Json(c).into_response();
        }
        slowotlok_backend_rust::repo::RepositorySimpleResult::Failed(error_text) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error_text).into_response();
        }
    }
}

async fn import_cards() -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
}

async fn get_cards_random(
    State(repo): State<Arc<RwLock<Repository>>>,
    Path(count): Path<usize>,
) -> Response {
    let out = repo.read().unwrap().random(count);
    Json(out).into_response()
}

// async fn get_cards_random_lowest(Path(count): Path<usize>) -> Response {
//     (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
// }

#[derive(Deserialize)]
struct AddCardDTO {
    src: String,
    tr: String,
}

#[derive(Deserialize)]
struct CardDTO {
    id: i64,
    src: String,
    tr: String,
    good: u32,
    bad: u32,
}

#[derive(Deserialize)]
struct ErrorDTO {
    message: String,
}
