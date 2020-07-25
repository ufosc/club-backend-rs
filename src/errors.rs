use actix_web::{error::ResponseError, HttpResponse};
use diesel::r2d2::{Error as R2D2Error};
use diesel::result::Error as DbResultError;
use derive_more::Display;
use std::convert::From;
use diesel::result::DatabaseErrorKind;

#[derive(Debug, Display)]
pub enum Errors {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Unauthorized")]
    LoginFailed,
    #[display(fmt = "Bad Request {}", _0)]
    BadRequest(String),
}

impl ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        match self {
            Errors::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try later"),
            Errors::LoginFailed => HttpResponse::Unauthorized().json("Bad username/password."),
            Errors::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
        }
    }
}

impl From<R2D2Error> for Errors {
    fn from(_: R2D2Error) -> Self {
        Errors::InternalServerError
    }
}

impl From<DbResultError> for Errors {
    fn from(error: DbResultError) -> Self {
        match error {
            DbResultError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message =
                        info.details().unwrap_or_else(|| info.message()).to_string();
                    return Errors::BadRequest(message);
                }
                Errors::InternalServerError
            }
            _ => Errors::InternalServerError,
        }
    }
}
