// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use log;
use rocket::Outcome;
use rocket::http::{Status, Cookies, Cookie};
use rocket::request::{self, Request, FromRequest};
use rand::{self, Rng};
use sha2::{Sha256, Digest};

use APP_CONF;

pub struct AuthGuard(pub i32);
pub struct AuthAnonymousGuard;

pub static AUTH_USER_COOKIE_NAME: &'static str = "user_id";

impl<'a, 'r> FromRequest<'a, 'r> for AuthGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthGuard, ()> {
        if let Outcome::Success(cookies) = request.guard::<Cookies>() {
            if let Some(user_id_cookie) = read(cookies) {
                if let Ok(user_id) = user_id_cookie.value().parse::<i32>() {
                    log::debug!("got user_id from cookies: {}", &user_id);

                    return Outcome::Success(AuthGuard(user_id));
                }
            }
        }

        Outcome::Failure((Status::Forbidden, ()))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthAnonymousGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthAnonymousGuard, ()> {
        match request.guard::<AuthGuard>() {
            Outcome::Success(_) => Outcome::Failure((Status::Gone, ())),
            _ => Outcome::Success(AuthAnonymousGuard),
        }
    }
}

pub fn insert(mut cookies: Cookies, user_id: String) {
    cookies.add_private(Cookie::new(AUTH_USER_COOKIE_NAME, user_id));
}

pub fn cleanup(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named(AUTH_USER_COOKIE_NAME));
}

fn read(mut cookies: Cookies) -> Option<Cookie> {
    cookies.get_private(AUTH_USER_COOKIE_NAME)
}

pub fn password_encode(password: &str) -> Vec<u8> {
    let password_salted = [password, APP_CONF.database.password_salt.as_str()].join("");

    log::debug!(
        "salted password: {} and got result: {}",
        password,
        &password_salted
    );

    let mut hasher = Sha256::default();

    hasher.input(&password_salted.into_bytes());

    hasher.result().to_vec()
}

pub fn password_verify(reference: &[u8], password: &str) -> bool {
    let password_encoded = password_encode(password);

    password_encoded == reference
}

pub fn recovery_generate() -> (Vec<u8>, String) {
    let recovery_password = rand::thread_rng()
        .gen_ascii_chars()
        .take(40)
        .collect::<String>();

    (password_encode(&recovery_password), recovery_password)
}
