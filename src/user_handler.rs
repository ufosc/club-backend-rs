use actix_web::{get, Responder, web};

use crate::models::User;

#[get("/")]
pub async fn get_users() -> impl Responder {
    serde_json::to_string(&vec![
        User {
            user_id: 1,
            username: "test2".to_string(),
            email: None,
            password_hash: None,
        },
        User {
            user_id: 2,
            username: "test1".to_string(),
            email: None,
            password_hash: None,
        },
    ]).unwrap()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
}
