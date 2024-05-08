use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::card)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CardEntity {
    pub id: i32,
    pub src: String,
    pub tr: String,
    pub good: i32,
    pub bad: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::card)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCard {
    pub src: String,
    pub tr: String,
    good: i32,
    bad: i32
}

impl NewCard {
    pub fn new(src: String, tr: String) -> Self{
        NewCard{src, tr, good: 0, bad: 0}
    }
}