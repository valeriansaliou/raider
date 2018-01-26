// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use log;
use rocket::Outcome;
use rocket::http::{Status, Cookies, Cookie};
use rocket::request::{self, Request, FromRequest};

pub struct AuthGuard;
pub struct AuthAnonymousGuard;

pub static AUTH_USER_COOKIE_NAME: &'static str = "user_id";

impl<'a, 'r> FromRequest<'a, 'r> for AuthGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthGuard, ()> {
        if let Outcome::Success(cookies) = request.guard::<Cookies>() {
            if let Some(user_id) = read(cookies) {
                log::debug!("got user_id from cookies: {}", &user_id);

                // TODO: validate user ID against DB

                return Outcome::Success(AuthGuard);
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
