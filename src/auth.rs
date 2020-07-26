use bcrypt::{verify, DEFAULT_COST, hash, BcryptResult};
use jsonwebtoken::{Algorithm, encode, Header, EncodingKey};
use log::{debug, error};
use crate::models::{RegisterRequest, NewUser, User, TokenClaims};

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

fn make_claim(user: &User) -> TokenClaims {
    TokenClaims {
        user_id: user.user_id,
    }
}

pub fn sign_token(user: &User, signing_key: &String) -> Option<String> {
    match encode(&Header::new(Algorithm::HS512), &make_claim(user), &EncodingKey::from_secret(signing_key.as_bytes())) {
        Ok(token) => Some(token),
        Err(e) => {
            error!("Error encoding JWT token, {}", e);
            None
        }
    }
}
