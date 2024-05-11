use diesel::{
    query_dsl::methods::FilterDsl, query_source::AppearsInFromClause, Connection,
    ExpressionMethods, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use dotenvy::dotenv;
use std::env;

use crate::{
    apperror::AppError,
    models::{Card, NewCard},
    repository::RepositoryTrait,
    schema,
};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{database_url}");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct RepositorySqlite {
    con: SqliteConnection,
}

impl RepositorySqlite {
    pub fn new() -> RepositorySqlite {
        RepositorySqlite {
            con: establish_connection(),
        }
        // RepositorySqlite{manager: ConnectionManager::<SqliteConnection>::new("db.sqlite")}
    }
}

impl RepositoryTrait for RepositorySqlite {
    fn random(&self, _count: usize) -> anyhow::Result<Vec<Card>, AppError> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }

    fn all(&mut self) -> anyhow::Result<Vec<Card>> {
        // use self::schema::card::dsl::*;
        Ok(self::schema::card::dsl::card.load::<Card>(&mut self.con)?)
    }

    fn insert(&mut self, card: NewCard) -> anyhow::Result<Card> {
        let card = diesel::insert_into(schema::card::table)
            .values(card)
            .returning(Card::as_returning())
            .get_result::<Card>(&mut self.con)?;
        Ok(card)
    }

    fn update(&mut self, card: Card) -> anyhow::Result<Card> {
        let card = diesel::update(schema::card::table)
            .set(card)
            .returning(Card::as_returning())
            .get_result::<Card>(&mut self.con)?;
        Ok(card)
    }

    fn delete(&mut self, del_id: i32) -> Result<i32, String> {
        use self::schema::card::dsl::*;
        if let Err(error) = diesel::delete(card.filter(id.eq(id))).execute(&mut self.con) {
            return Err(error.to_string());
        }
        Ok(del_id)
    }
}
