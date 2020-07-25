use diesel::{Insertable, Queryable};
use serde::Serialize;

use crate::schema::users;

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    #[serde(skip)]
    pub email: Option<String>,
    #[serde(skip)]
    pub password_hash: Option<String>,
}
