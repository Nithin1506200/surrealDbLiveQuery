use super::{Db, DB};
use surrealdb::opt::RecordId;

pub struct Accounts {
    email: String,
    id: RecordId,
    name: String,
}

pub fn create() {}
pub fn get_by_id(id: &str) {
    // DB.query("")
}
pub fn get_by_email(email: &str) {}
