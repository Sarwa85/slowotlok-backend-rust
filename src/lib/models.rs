use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::card)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CardEntity {
    pub id: i32,
    pub src: String,
    pub tr: String,
    pub good: i32,
    pub bad: i32,
}