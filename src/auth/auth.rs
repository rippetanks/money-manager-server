
extern crate crypto;
extern crate jwt;

use rocket::{Outcome, State};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use ring::{rand, pbkdf2, digest};
use ring::rand::SecureRandom;
use data_encoding::HEXUPPER;
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};

use crate::controller::Extras;
use crate::user::model::User;
use crate::auth::model::Auth;
use crate::database::MoneyManagerDB;
use rocket::outcome::IntoOutcome;
use std::borrow::Borrow;

pub struct ApiKey {
    pub sub: i64
}

const DEFAULT_ITERATION: i16 = 1000;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
    Broken
}

///
///
pub fn create_auth(email: &String, password: &String, iteration: Option<i16>, id_user: i64) -> Result<Auth, ()> {
    let it: u32 = iteration.unwrap_or(DEFAULT_ITERATION) as u32;
    let rng = rand::SystemRandom::new();
    // salt
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt).unwrap();
    // salted password
    let mut salted_pwd = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(&digest::SHA256, it, &salt, (*password).as_bytes(), &mut salted_pwd);
    // auth
    let salt_str = HEXUPPER.encode(&salt);
    trace!("salt_str: {}", salt_str);
    let stored_key = HEXUPPER.encode(&salted_pwd);
    trace!("stored_key: {}", stored_key);
    Ok(Auth {
        id: id_user,
        email: email.clone(),
        iteration: it as i16,
        salt: salt_str,
        stored_key: HEXUPPER.encode(&salted_pwd),
        last_login: None
    })
}

///
///
pub fn create_token(user: &User, extra: &Extras) -> Result<String, Status> {
    trace!("extras: {:?}", extra);
    let header: Header = Default::default();
    let claims = Registered {
        sub: Some(user.id.to_string()),
        exp: Some(extra.jwt_exp),
        ..Default::default()
    };
    let token = Token::new(header, claims);
    token.signed(extra.jwt_key.as_bytes(), Sha256::new())
        .map(|token| token)
        .map_err(|e| {
            error!("Can not generate token caused by {:?}", e);
            Status::InternalServerError
        })
}

///
///
#[allow(unused_must_use)]
pub fn login(auth: &Auth, pwd: &String, conn: &MoneyManagerDB) -> bool {
    let result = pbkdf2::verify(&digest::SHA256,
                                auth.iteration as u32,
                                HEXUPPER.decode(auth.salt.as_bytes()).unwrap().as_slice(),
                                (*pwd).as_bytes(),
                                HEXUPPER.decode(auth.stored_key.as_bytes()).unwrap().as_slice());
    result.map(|_| {
        debug!("update last login for user {}", auth.id);
        if !Auth::update_last_login(auth, conn) {
            error!("Can not update last login for user {}", auth.id);
        }
    });
    result.is_ok()
}

// #################################################################################################

fn read_token(key: &str, secret: &String) -> Result<ApiKey, String> {
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|e| {
            error!("can not parse key {:?}", e);
            "Unable to parse key".to_string()
        })?;
    // verify token
    if token.verify(secret.as_bytes(), Sha256::new()) {
        Ok(ApiKey {
          sub: token.claims.sub.ok_or("sub not valid".to_string())?.parse::<i64>().unwrap()
        })
    } else {
        error!("token invalid {:?}", token);
        Err("Token not valid".to_string())
    }
}

// #################################################################################################

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        let extra = request.guard::<State<Extras>>().unwrap();
        match keys.len() {
            0 => {
                warn!("Access denied! Missing API KEY.");
                Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
            },
            1 => match read_token(keys[0], &extra.jwt_key) {
                Ok(api_key) => Outcome::Success(api_key),
                Err(_) => {
                    warn!("Access denied! Invalid API KEY.");
                    Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                }
            },
            _ => {
                warn!("Access denied! Too much API KEY.");
                Outcome::Failure((Status::Unauthorized, ApiKeyError::BadCount))
            }
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        let conn = request.guard::<MoneyManagerDB>().unwrap();
        let key_outcome = request.guard::<ApiKey>();
        if key_outcome.is_failure() {
            // forward failure from ApiKey handler
            return Outcome::Failure(key_outcome.failed().unwrap());
        }
        let key = key_outcome.unwrap();
        let user = User::read_by_id(key.sub, &conn);
        match user {
            Ok(user) => {
                debug!("Access granted to user {}", user.id);
                Outcome::Success(user)
            },
            Err(e) => {
                warn!("Access denied to user {} caused by {}", key.sub, e.to_string());
                Outcome::Failure((Status::Unauthorized, ApiKeyError::Broken))
            }
        }
    }
}
