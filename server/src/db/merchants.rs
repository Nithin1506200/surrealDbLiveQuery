use surrealdb::opt::RecordId;

use super::{Db, DB};

pub struct Merchants {
    email: String,
    id: RecordId,
    name: String,
}

pub fn create() {}
pub fn get_by_id(id: &str) {
    // DB.query("")
}
pub fn get_by_email(email: &str) {}
