use actix_web::{post, Responder, web};
use actix_web::error::BlockingError;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::auth;
use crate::errors::Errors;
use crate::models::{LoginRequest, Pool, RegisterRequest, User, JWT};
use crate::schema::users::dsl::{username, users};

#[post("/register")]
pub async fn register(register_req: web::Json<RegisterRequest>, pool: web::Data<Pool>, signing_key: web::Data<String>) -> impl Responder {
    match web::block(move || insert_user(register_req.into_inner(), pool)).await {
        Ok(user) => match auth::sign_token(&user, signing_key.get_ref()) {
            Some(token) => Ok(JWT {token}),
            None => Err(Errors::InternalServerError)
        },
        Err(e) => match e {
            BlockingError::Error(service_error) => Err(Errors::from(service_error)),
            BlockingError::Canceled => Err(Errors::InternalServerError),
        },
    }
}

#[post("/login")]
pub async fn login(login_req: web::Json<LoginRequest>, pool: web::Data<Pool>, signing_key: web::Data<String>) -> impl Responder {
    match web::block(move || query_user(login_req.into_inner(), pool)).await {
        Ok(user) => match auth::sign_token(&user, signing_key.get_ref()) {
            Some(token) => Ok(JWT {token}),
            None => Err(Errors::InternalServerError)
        },
        Err(e) => match e {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(Errors::InternalServerError),
        },
    }
}

fn query_user(login_req: LoginRequest, pool: web::Data<Pool>) -> Result<User, Errors> {
    let conn = &pool.get().expect("Could not get connection pool");

    let mut user_vec: Vec<User> = users
        .filter(username.eq(&login_req.username))
        .limit(1)
        .load::<User>(&*conn)
        .expect("Could not get User by username");

    match user_vec.pop() {
        Some(user) => match auth::password_matches(&login_req.password, (&user.password_hash).as_ref().unwrap_or(&String::from(""))) {
            true => Ok(user.clone()),
            false => Err(Errors::LoginFailed),
        },
        None => Err(Errors::LoginFailed),
    }
}

fn insert_user(register_req: RegisterRequest, pool: web::Data<Pool>) -> Result<User, diesel::result::Error> {
    let conn = &pool.get().expect("Could not get connection pool");

    diesel::insert_into(crate::schema::users::table)
        .values(auth::create_user(register_req))
        .get_result(&*conn)
        // .expect("Error inserting new user into database.")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(login)
        .service(register);
}
