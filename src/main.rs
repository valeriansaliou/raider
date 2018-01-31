// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

#![feature(use_extern_macros, custom_derive, plugin)]
#![plugin(rocket_codegen)]

#[macro_use(log)]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate sha2;
extern crate time;
extern crate rand;
extern crate validate;
extern crate toml;
extern crate base64;
extern crate url_serde;
extern crate chrono;
extern crate native_tls;
extern crate openssl_probe;
extern crate lettre;
extern crate lettre_email;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate reqwest;
extern crate bigdecimal;
extern crate num_traits;
extern crate separator;
extern crate iso_country;

mod config;
mod exchange;
mod notifier;
mod responder;
mod storage;
mod track;

use std::thread;
use std::ops::Deref;
use std::str::FromStr;
use std::time::Duration;

use clap::{App, Arg};
use log::LogLevelFilter;

use config::config::Config;
use config::logger::ConfigLogger;
use config::reader::ConfigReader;
use exchange::manager::run as run_exchange;
use responder::manager::run as run_responder;

struct AppArgs {
    config: String,
}

pub static THREAD_NAME_EXCHANGE: &'static str = "raider-exchange";
pub static THREAD_NAME_RESPONDER: &'static str = "raider-responder";

macro_rules! gen_spawn_managed {
    ($name:expr, $method:ident, $thread_name:ident, $managed_fn:ident) => (
        fn $method() {
            log::debug!("spawn managed thread: {}", $name);

            let worker = thread::Builder::new()
                .name($thread_name.to_string())
                .spawn($managed_fn);

            // Block on worker thread (join it)
            let has_error = if let Ok(worker_thread) = worker {
                worker_thread.join().is_err()
            } else {
                true
            };

            // Worker thread crashed?
            if has_error == true {
                log::error!("managed thread crashed ({}), setting it up again", $name);

                // Prevents thread start loop floods
                thread::sleep(Duration::from_secs(1));

                $method();
            }
        }
    )
}

lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
    static ref APP_CONF: Config = ConfigReader::make();
}

gen_spawn_managed!(
    "exchange",
    spawn_exchange,
    THREAD_NAME_EXCHANGE,
    run_exchange
);
gen_spawn_managed!(
    "responder",
    spawn_responder,
    THREAD_NAME_RESPONDER,
    run_responder
);

fn make_app_args() -> AppArgs {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Path to configuration file")
                .default_value("./config.cfg")
                .takes_value(true),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs { config: String::from(matches.value_of("config").expect("invalid config value")) }
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    APP_ARGS.deref();
    APP_CONF.deref();

    // Ensure assets path exists
    assert_eq!(
        APP_CONF.assets.path.exists(),
        true,
        "assets directory not found: {:?}",
        APP_CONF.assets.path
    );
}

fn main() {
    // Ensure OpenSSL root chain is found on current environment
    openssl_probe::init_ssl_cert_env_vars();

    // Initialize shared logger
    let _logger = ConfigLogger::init(
        LogLevelFilter::from_str(&APP_CONF.server.log_level).expect("invalid log level"),
    );

    log::info!("starting up");

    // Ensure all states are bound
    ensure_states();

    // Spawn exchange (background thread)
    thread::spawn(spawn_exchange);

    // Spawn Web responder (foreground thread)
    spawn_responder();

    log::error!("could not start");
}
