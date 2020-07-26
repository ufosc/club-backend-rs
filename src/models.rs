use actix_web::{Error, Responder, HttpResponse, HttpRequest};
use diesel::{Insertable, PgConnection, Queryable};
use diesel::r2d2::ConnectionManager;
use futures::future::{ready, Ready};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub token: String,
}

impl Responder for JWT {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
