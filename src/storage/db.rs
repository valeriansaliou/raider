// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;
use std::ops::Deref;
use log;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::mysql::MysqlConnection;

use APP_CONF;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for DbConn {
    type Target = MysqlConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub fn pool() -> Pool {
    log::debug!("setting up db pool...");

    let manager = ConnectionManager::<MysqlConnection>::new(APP_CONF.database.url.as_str());

    let pool = r2d2::Pool::builder()
        .max_size(APP_CONF.database.pool_size)
        .idle_timeout(Some(Duration::from_secs(APP_CONF.database.idle_timeout)))
        .connection_timeout(Duration::from_secs(APP_CONF.database.connection_timeout))
        .build(manager)
        .expect("db pool");

    log::debug!("db pool configured");

    pool
}
