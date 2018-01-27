// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::path::PathBuf;
use log;
use time;
use chrono::naive::NaiveDateTime;
use validate::rules::{email as validate_email};
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::Cookies;
use rocket_contrib::Template;
use diesel;
use diesel::prelude::*;

use super::context::{CONFIG_CONTEXT, ConfigContext};
use super::asset_file::AssetFile;
use super::auth_guard::{
    AuthGuard,
    AuthAnonymousGuard,
    cleanup as auth_cleanup,
    insert as auth_insert,
    password_verify as auth_password_verify,
    password_encode as auth_password_encode,
    recovery_generate as auth_recovery_generate
};
use notifier::email::EmailNotifier;
use storage::db::DbConn;
use storage::schemas::account::dsl::{
    account,
    id as account_id,
    email as account_email,
    password as account_password,
    recovery as account_recovery,
    created_at as account_created_at,
    updated_at as account_updated_at
};
use storage::models::{Account, AccountRecoveryUpdate};
use APP_CONF;

#[derive(FromForm)]
pub struct InitiateArgs {
    result: String,
}

#[derive(FromForm)]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(FromForm)]
pub struct SignupData {
    email: String,
    password: String,
    password_repeat: String,
}

#[derive(FromForm)]
pub struct RecoverData {
    email: String,
}

#[derive(Serialize)]
pub struct LoginContext<'a> {
    pub failure: bool,
    pub config: &'a ConfigContext
}

#[derive(Serialize)]
pub struct SignupContext<'a> {
    pub failure: bool,
    pub config: &'a ConfigContext
}

#[derive(Serialize)]
pub struct RecoverContext<'a> {
    pub success: bool,
    pub failure: bool,
    pub config: &'a ConfigContext
}

#[derive(Serialize)]
pub struct DashboardBaseContext<'a> {
    pub config: &'a ConfigContext
}

#[derive(Serialize)]
pub struct DashboardTrackersContext<'a> {
    pub config: &'a ConfigContext
}

#[derive(Serialize)]
pub struct DashboardPayoutsContext<'a> {
    pub config: &'a ConfigContext
}

#[derive(Serialize)]
pub struct DashboardAccountContext<'a> {
    pub config: &'a ConfigContext
}

#[get("/")]
fn get_index(_anon: AuthAnonymousGuard) -> Redirect {
    Redirect::found("/initiate/")
}

#[get("/initiate")]
fn get_initiate_base(_anon: AuthAnonymousGuard) -> Redirect {
    Redirect::found("/initiate/login/")
}

#[get("/initiate/login")]
fn get_initiate_login(_anon: AuthAnonymousGuard) -> Template {
    Template::render("initiate_login", &LoginContext {
        failure: false,
        config: &CONFIG_CONTEXT
    })
}

#[get("/initiate/login?<args>")]
fn get_initiate_login_args(_anon: AuthAnonymousGuard, args: InitiateArgs) -> Template {
    Template::render("initiate_login", &LoginContext {
        failure: args.result == "failure",
        config: &CONFIG_CONTEXT
    })
}

#[post("/initiate/login", data = "<data>")]
fn post_initiate_login(
    _anon: AuthAnonymousGuard,
    cookies: Cookies,
    db: DbConn,
    data: Form<LoginData>
) -> Redirect {
    let data_inner = data.get();

    if data_inner.email.is_empty() == false && data_inner.password.is_empty() == false &&
        validate_email().validate(&data_inner.email).is_ok() == true {
        let account_result = account
            .filter(account_email.eq(&data_inner.email))
            .first::<Account>(&*db);

        match account_result {
            Ok(result) => {
                let mut is_auth_valid = auth_password_verify(
                    &result.password, &data_inner.password
                );

                // Attempt to check recovery password?
                if is_auth_valid == false {
                    if let Some(ref recovery) = result.recovery {
                        is_auth_valid = auth_password_verify(recovery, &data_inner.password);
                    }
                }

                // Password is valid?
                if is_auth_valid == true {
                    // Erase any stored recovery password?
                    if result.recovery.is_some() == true {
                        let recovery_update = diesel::update(
                            account.filter(account_id.eq(result.id))
                        )
                            .set(&AccountRecoveryUpdate {
                                recovery: Vec::new()
                            })
                            .execute(&*db);

                        match recovery_update {
                            Ok(_) => log::info!("cleared recovery password"),
                            Err(err) => log::error!("failed clearing recovery password: {}", err),
                        }
                    }

                    // Log-in user (set cookie)
                    auth_insert(cookies, result.id.to_string());

                    return Redirect::to("/dashboard/")
                }
            },
            Err(err) => {
                log::debug!("account not retrieved for login: {}", err);
            },
        };
    }

    Redirect::to("/initiate/login/?result=failure")
}

#[get("/initiate/signup")]
fn get_initiate_signup(_anon: AuthAnonymousGuard) -> Template {
    Template::render("initiate_signup", &SignupContext {
        failure: false,
        config: &CONFIG_CONTEXT
    })
}

#[get("/initiate/signup?<args>")]
fn get_initiate_signup_args(_anon: AuthAnonymousGuard, args: InitiateArgs) -> Template {
    Template::render("initiate_signup", &SignupContext {
        failure: args.result == "failure",
        config: &CONFIG_CONTEXT
    })
}

#[post("/initiate/signup", data = "<data>")]
fn post_initiate_signup(
    _anon: AuthAnonymousGuard,
    cookies: Cookies,
    db: DbConn,
    data: Form<SignupData>
) -> Redirect {
    let data_inner = data.get();

    if data_inner.email.is_empty() == false && data_inner.password.is_empty() == false &&
        validate_email().validate(&data_inner.email).is_ok() == true &&
        data_inner.password == data_inner.password_repeat {
        let now_date = NaiveDateTime::from_timestamp(time::now().tm_sec as i64, 0);

        let insert_result = diesel::insert_into(account)
            .values((
                &account_email.eq(&data_inner.email),
                &account_password.eq(&auth_password_encode(&data_inner.password)),
                &account_created_at.eq(&now_date),
                &account_updated_at.eq(&now_date)
            ))
            .execute(&*db);

        if insert_result.is_ok() == true {
            let account_result = account
                .filter(account_email.eq(&data_inner.email))
                .first::<Account>(&*db);

            match account_result {
                Ok(result) => {
                    // Log-in user (set cookie)
                    auth_insert(cookies, result.id.to_string());

                    return Redirect::to("/dashboard/");
                },
                Err(err) => {
                    log::debug!("account not retrieved for login: {}", err);
                },
            };
        }
    }

    Redirect::to("/initiate/signup/?result=failure")
}

#[get("/initiate/recover")]
fn get_initiate_recover(_anon: AuthAnonymousGuard) -> Template {
    Template::render("initiate_recover", &RecoverContext {
        failure: false,
        success: false,
        config: &CONFIG_CONTEXT
    })
}

#[get("/initiate/recover?<args>")]
fn get_initiate_recover_args(_anon: AuthAnonymousGuard, args: InitiateArgs) -> Template {
    Template::render("initiate_recover", &RecoverContext {
        failure: args.result == "failure",
        success: args.result == "success",
        config: &CONFIG_CONTEXT
    })
}

#[post("/initiate/recover", data = "<data>")]
fn post_initiate_recover(
    _anon: AuthAnonymousGuard,
    db: DbConn,
    data: Form<RecoverData>
) -> Redirect {
    let data_inner = data.get();

    if data_inner.email.is_empty() == false &&
        validate_email().validate(&data_inner.email).is_ok() == true {
        let result = account
            .filter(account_email.eq(&data_inner.email))
            .first::<Account>(&*db);

        if let Ok(account_result) = result {
            let recovery_params = auth_recovery_generate();

            let recovery_result = diesel::update(account.filter(account_id.eq(account_result.id)))
                .set(account_recovery.eq(Some(&recovery_params.0)))
                .execute(&*db);

            if recovery_result.is_ok() == true {
                log::debug!(
                    "will send recovery email to: {} with password: {}",
                    &account_result.email,
                    recovery_params.1
                );

                // Generate password recovery message
                let mut message = String::new();

                message.push_str("Hi,\n\n");

                message.push_str(&format!(
                    "A password recovery has been requested on your {} account.\n",
                    &APP_CONF.branding.page_title
                ));

                message.push_str(&format!(
                    "Please login with this password to access your dashboard: {}\n\n",
                    recovery_params.1
                ));

                message.push_str(
                    "Your account main password was not changed. Please update it once logged in."
                );

                // Send password recovery email
                if EmailNotifier::dispatch(
                    &account_result.email, "Recover your password".to_string(), &message
                ).is_ok() == true {
                    return Redirect::to("/initiate/recover/?result=success");
                }
            }
        }
    }

    Redirect::to("/initiate/recover/?result=failure")
}

#[get("/initiate/logout")]
fn get_initiate_logout(_auth: AuthGuard, cookies: Cookies) -> Redirect {
    auth_cleanup(cookies);

    Redirect::to("/initiate/")
}

#[get("/dashboard")]
fn get_dashboard_base(_auth: AuthGuard) -> Template {
    Template::render("dashboard_base", &DashboardBaseContext {
        config: &CONFIG_CONTEXT
    })
}

#[get("/dashboard/trackers")]
fn get_dashboard_trackers(_auth: AuthGuard) -> Template {
    Template::render("dashboard_trackers", &DashboardTrackersContext {
        config: &CONFIG_CONTEXT
    })
}

#[get("/dashboard/payouts")]
fn get_dashboard_payouts(_auth: AuthGuard) -> Template {
    Template::render("dashboard_payouts", &DashboardPayoutsContext {
        config: &CONFIG_CONTEXT
    })
}

#[get("/dashboard/account")]
fn get_dashboard_account(_auth: AuthGuard) -> Template {
    Template::render("dashboard_account", &DashboardAccountContext {
        config: &CONFIG_CONTEXT
    })
}

#[get("/robots.txt")]
fn get_robots() -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./public/robots.txt")).ok()
}

#[get("/assets/fonts/<file..>")]
fn get_assets_fonts(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./fonts").join(file)).ok()
}

#[get("/assets/images/<file..>")]
fn get_assets_images(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./images").join(file)).ok()
}

#[get("/assets/stylesheets/<file..>")]
fn get_assets_stylesheets(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./stylesheets").join(file)).ok()
}
