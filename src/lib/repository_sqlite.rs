use diesel::{connection, query_dsl::methods::SelectDsl, Connection, QueryDsl, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
// use r2d2_diesel::ConnectionManager;
use std::env;

use crate::{models::CardEntity, repository::RepositoryTrait, schema};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{database_url}");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct RepositorySqlite {
    con: SqliteConnection
}

impl RepositorySqlite {
    pub fn new() -> RepositorySqlite{
        RepositorySqlite{con: establish_connection()}
        // RepositorySqlite{manager: ConnectionManager::<SqliteConnection>::new("db.sqlite")}
    }
}

impl RepositoryTrait for RepositorySqlite {
    fn random(&self, count: usize) -> Vec<CardEntity> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }

    fn all(&mut self) -> Vec<CardEntity> {
        use self::schema::card::dsl::*;
        return card.load::<CardEntity>(&mut self.con).unwrap();
    }

    fn insert(&mut self, c: &mut CardEntity) -> crate::simple_repository::RepositorySimpleResult {
        use self::schema::card::dsl::*;
        let cnew = CardEntity{id: c.id, src: c.src.clone(), tr: c.tr.clone(), good: c.good, bad: c.bad};
        diesel::insert_into(card).values(cnew).execute(&mut self.con).unwrap();
        crate::simple_repository::RepositorySimpleResult::OK
        // todo!()
    }

    fn insert_list(&mut self, cards: &mut Vec<CardEntity>) -> crate::simple_repository::RepositorySimpleResult {
        todo!()
    }

    fn update(&mut self, card: &CardEntity) -> crate::simple_repository::RepositorySimpleResult {
        todo!()
    }

    fn delete(&mut self, card: &CardEntity) -> crate::simple_repository::RepositorySimpleResult {
        todo!()
    }

    fn delete_by_id(&mut self, id: i64) -> crate::simple_repository::RepositorySimpleResult {
        todo!()
    }
}