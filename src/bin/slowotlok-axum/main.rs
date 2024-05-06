use std::{
    borrow::{Borrow, BorrowMut},
    string,
    sync::{Arc, Mutex, RwLock},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Error, Json, Router,
};
use cursive::vec;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use serde::{Deserialize, Serialize};
use slowotlok_backend::{repository_sqlite::{establish_connection, RepositorySqlite}};
use slowotlok_backend::repository::RepositoryTrait;
use slowotlok_backend::dtos::CardDTO;


// fn main1() {
//     let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
//     let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

//     for _ in 0..10i32 {
//         let pool = pool.clone();
//         thread::spawn(move || {
//             let connection = pool.get();

//             assert!(connection.is_ok());
//         });
//     }
// }




#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // let mut repo: Arc<RwLock<dyn RepositoryTrait + Sync + Send>> = Arc::new(RwLock::new(SimpleRepository::new()));
    let mut repo = Arc::new(Mutex::new(RepositorySqlite::new()));





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

async fn handle_error(error: Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", error),
    )
}

async fn add_card(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Json(payload): Json<AddCardDTO>,
) -> Response {
    println!("Adding card...");
    let mut e = CardDTO::to_entity(CardDTO::new(payload.src, payload.tr));
    let mut r = repo.lock().unwrap();
    match r.insert(&mut e) {
        slowotlok_backend::simple_repository::RepositorySimpleResult::OK => {
            return Json(CardDTO::from_entity(&e)).into_response();
        }
        slowotlok_backend::simple_repository::RepositorySimpleResult::Failed(error_text) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error_text).into_response();
        }
    }
}

async fn rm_card(State(repo): State<Arc<Mutex<impl RepositoryTrait>>>, Path(id): Path<i64>) -> Response {
    // let mut c = Card::new(payload.src, payload.tr);
    let mut r = repo.lock().unwrap();
    match r.delete_by_id(id) {
        slowotlok_backend::simple_repository::RepositorySimpleResult::OK => {
            return Response::new("".into());
        }
        slowotlok_backend::simple_repository::RepositorySimpleResult::Failed(error_text) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error_text).into_response();
        }
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
    let mut response_struct = ImportCardsResponse{added: vec![], errors: vec![]};
    // let mut cards_added: Vec<Card> = vec![];
    // let mut error_text_list: Vec<String> = vec![];
    for card in payload.iter() {
        let mut e = CardDTO::to_entity(CardDTO::new(card.src.clone(),card.tr.clone()));
        let mut r = repo.lock().unwrap();
        match r.insert(&mut e) {
            slowotlok_backend::simple_repository::RepositorySimpleResult::OK => {
                response_struct.added.push(CardDTO::from_entity(&e));
            },
            slowotlok_backend::simple_repository::RepositorySimpleResult::Failed(error_text) => {
                response_struct.errors.push(error_text)
            },
        }
    }
    Json(response_struct).into_response()
    // (StatusCode::NOT_IMPLEMENTED, "Not implemented yet").into_response()
}

async fn get_cards_random(
    State(repo): State<Arc<Mutex<impl RepositoryTrait>>>,
    Path(count): Path<usize>,
) -> Response {
    let r = repo.lock().unwrap();
    let out: Vec<_> = r.random(count).iter().map(|x| CardDTO::from_entity(x)).collect();
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

// #[derive(Deserialize)]
// struct ErrorDTO {
//     message: String,
// }

#[derive(Serialize)]
struct ImportCardsResponse {
    added: Vec<CardDTO>,
    errors: Vec<String>,
}