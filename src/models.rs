#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub used: bool
}

use super::schema::*;

#[derive(Insertable)]
#[table_name="words"]
pub struct NewWord<'a> {
    pub word: &'a str
}