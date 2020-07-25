use diesel::{Insertable, PgConnection, Queryable};
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};

use crate::schema::users;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone, Insertable, Queryable, Serialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    #[serde(skip)]
    pub email: Option<String>,
    #[serde(skip)]
    pub password_hash: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}
