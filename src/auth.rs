use bcrypt::{verify, DEFAULT_COST, hash, BcryptResult};
use log::{debug, error};
use crate::models::{RegisterRequest, NewUser};

pub fn password_matches(password: &String, password_hash: &String) -> bool {
    debug!("Verifying password");
    match verify(password, password_hash) {
        Ok(does_match) => {debug!("Verified password"); does_match},
        Err(e) => {
            error!("Error verifying password hash {}", e);
            false
        },
    }
}

pub fn create_user(register_req: RegisterRequest) -> Option<NewUser> {
    let hashed_pass = hash_password(&register_req.password);
    match hashed_pass {
        Ok(password_hash) => Some(NewUser {
            username: register_req.username,
            password_hash,
            email: register_req.email,
        }),
        Err(e) => {
            error!("Error hashing password, {}", e);
            None
        },
    }
}

fn hash_password(password: &String) -> BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

