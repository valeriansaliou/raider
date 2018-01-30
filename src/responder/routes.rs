// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::path::PathBuf;
use std::collections::HashSet;
use log;
use chrono::offset::Utc;
use validate::rules::{email as validate_email};
use separator::{Separatable, FixedPlaceSeparatable};
use bigdecimal::BigDecimal;
use num_traits::cast::ToPrimitive;
use iso_country::data::{all as countries};
use rand::{self, Rng};
use rocket::error::{Error as RocketError};
use rocket::response::{Redirect, Failure};
use rocket::request::{Form, FromForm, FormItems, FromFormValue};
use rocket::http::{Cookies, Status};
use rocket_contrib::Template;
use diesel;
use diesel::prelude::*;
use diesel::dsl::{sum, count};

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
use super::utilities::{get_balance, check_argument_value};
use notifier::email::EmailNotifier;
use storage::db::DbConn;
use storage::schemas::account::dsl::{
    account,
    id as account_id,
    email as account_email,
    password as account_password,
    recovery as account_recovery,
    commission as account_commission,
    full_name as account_full_name,
    address as account_address,
    country as account_country,
    payout_method as account_payout_method,
    payout_instructions as account_payout_instructions,
    created_at as account_created_at,
    updated_at as account_updated_at
};
use storage::schemas::payout::dsl::{
    payout,
    account_id as payout_account_id,
    created_at as payout_created_at
};
use storage::schemas::tracker::dsl::{
    tracker,
    id as tracker_id,
    label as tracker_label,
    account_id as tracker_account_id,
    created_at as tracker_created_at,
    updated_at as tracker_updated_at
};
use storage::schemas::balance::dsl::{
    balance,
    account_id as balance_account_id,
    tracker_id as balance_tracker_id,
    amount as balance_amount
};
use storage::models::{Account, Payout, Tracker, AccountRecoveryUpdate};
use storage::choices::ACCOUNT_PAYOUT_METHODS;
use APP_CONF;

const PAYOUTS_LIMIT_PER_PAGE: i64 = 50;

#[derive(FromForm)]
pub struct InitiateArgs {
    result: Option<String>,
}

#[derive(FromForm)]
pub struct DashboardArgs {
    result: Option<String>,
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

#[derive(FromForm)]
pub struct DashboardTrackersFormCreateData {
    name: String
}

pub struct DashboardTrackersFormRemoveData {
    trackers: HashSet<String>
}

#[derive(FromForm)]
pub struct DashboardAccountFormAccountData {
    email: String,
    password: String,
}

#[derive(FromForm)]
pub struct DashboardAccountFormPayoutData {
    full_name: String,
    address: String,
    country: String,
    payout_method: String,
    payout_instructions: String,
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
pub struct DashboardCommonContext {
    pub balance_pending: String
}

#[derive(Serialize)]
pub struct DashboardBaseContext<'a> {
    pub common: DashboardCommonContext,
    pub config: &'a ConfigContext,
    pub has_trackers: bool,
    pub commission_percent: u8
}

#[derive(Serialize)]
pub struct DashboardTrackersContext<'a> {
    pub create_failure: bool,
    pub create_success: bool,
    pub remove_failure: bool,
    pub remove_neutral: bool,
    pub remove_success: bool,
    pub trackers: Vec<DashboardTrackersContextTracker>,
    pub common: DashboardCommonContext,
    pub config: &'a ConfigContext
}

#[derive(Serialize)]
pub struct DashboardTrackersContextTracker {
    pub tracking_id: String,
    pub label: String,
    pub statistics_signups: String,
    pub statistics_paying: String,
    pub total_earned: String
}

#[derive(Serialize)]
pub struct DashboardPayoutsContext<'a> {
    pub common: DashboardCommonContext,
    pub config: &'a ConfigContext,
    pub balance_total: String,
    pub payouts: Vec<DashboardPayoutsContextPayout>,
    pub has_more: bool
}

#[derive(Serialize)]
pub struct DashboardPayoutsContextPayout {
    pub number: i32,
    pub status: String,
    pub amount: String,
    pub currency: String,
    pub account: String,
    pub invoice_url: String,
    pub date: String
}

#[derive(Serialize)]
pub struct DashboardAccountContext<'a, 'b> {
    pub success: bool,
    pub neutral: bool,
    pub failure: bool,
    pub common: DashboardCommonContext,
    pub config: &'a ConfigContext,
    pub payout_methods: &'static [(&'static str, &'static str)],
    pub countries: Vec<(&'b str, &'b str)>,
    pub account: DashboardAccountContextAccount,
    pub payout: DashboardAccountContextPayout
}

#[derive(Serialize)]
pub struct DashboardAccountContextAccount {
    pub email: String
}

#[derive(Serialize)]
pub struct DashboardAccountContextPayout {
    pub full_name: String,
    pub address: String,
    pub country: String,
    pub method: String,
    pub instructions: String
}

impl DashboardCommonContext {
    fn build(db: &DbConn, user_id: i32) -> DashboardCommonContext {
        DashboardCommonContext {
            balance_pending: get_balance(db, user_id, Some(false))
        }
    }
}

impl<'f> FromForm<'f> for DashboardTrackersFormRemoveData {
    type Error = RocketError;

    fn from_form(form_items: &mut FormItems<'f>, _: bool) -> Result<Self, Self::Error> {
        let mut update = DashboardTrackersFormRemoveData {
            trackers: HashSet::new()
        };

        for (k, v) in form_items {
            let key: &str = &*k;
            let value = String::from_form_value(v)
                .or(Err(RocketError::BadParse))?;

            match key {
                "tracker" => update.trackers.insert(value),
                _ => {
                    return Err(RocketError::BadParse);
                }
            };
        }

        Ok(update)
    }
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
fn get_initiate_login(anon: AuthAnonymousGuard) -> Template {
    get_initiate_login_args(anon, InitiateArgs {
        result: None
    })
}

#[get("/initiate/login?<args>")]
fn get_initiate_login_args(_anon: AuthAnonymousGuard, args: InitiateArgs) -> Template {
    Template::render("initiate_login", &LoginContext {
        failure: check_argument_value(&args.result, "failure"),
        config: &CONFIG_CONTEXT
    })
}

#[post("/initiate/login/form/login", data = "<data>")]
fn post_initiate_login_form_login(
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
fn get_initiate_signup(anon: AuthAnonymousGuard) -> Template {
    get_initiate_signup_args(anon, InitiateArgs {
        result: None
    })
}

#[get("/initiate/signup?<args>")]
fn get_initiate_signup_args(_anon: AuthAnonymousGuard, args: InitiateArgs) -> Template {
    Template::render("initiate_signup", &SignupContext {
        failure: check_argument_value(&args.result, "failure"),
        config: &CONFIG_CONTEXT
    })
}

#[post("/initiate/signup/form/signup", data = "<data>")]
fn post_initiate_signup_form_signup(
    _anon: AuthAnonymousGuard,
    cookies: Cookies,
    db: DbConn,
    data: Form<SignupData>
) -> Redirect {
    let data_inner = data.get();

    if data_inner.email.is_empty() == false && data_inner.password.is_empty() == false &&
        validate_email().validate(&data_inner.email).is_ok() == true &&
        data_inner.password == data_inner.password_repeat {
        let now_date = Utc::now().naive_utc();

        let insert_result = diesel::insert_into(account)
            .values((
                &account_email.eq(&data_inner.email),
                &account_password.eq(&auth_password_encode(&data_inner.password)),
                &account_commission.eq(BigDecimal::from(APP_CONF.tracker.commission_default)),
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
fn get_initiate_recover(anon: AuthAnonymousGuard) -> Template {
    get_initiate_recover_args(anon, InitiateArgs {
        result: None
    })
}

#[get("/initiate/recover?<args>")]
fn get_initiate_recover_args(_anon: AuthAnonymousGuard, args: InitiateArgs) -> Template {
    Template::render("initiate_recover", &RecoverContext {
        failure: check_argument_value(&args.result, "failure"),
        success: check_argument_value(&args.result, "success"),
        config: &CONFIG_CONTEXT
    })
}

#[post("/initiate/recover/form/recover", data = "<data>")]
fn post_initiate_recover_form_recover(
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
fn get_dashboard_base(auth: AuthGuard, db: DbConn) -> Template {
    let account_result = account
        .filter(account_id.eq(auth.0))
        .first::<Account>(&*db);

    let tracker_count_result = tracker
        .filter(tracker_account_id.eq(auth.0))
        .select(count(tracker_id))
        .first(&*db);

    let commission_value = if let Ok(account_inner) = account_result {
        account_inner.commission.to_f32().unwrap_or(0.00)
    } else {
        0.00
    };

    Template::render("dashboard_base", &DashboardBaseContext {
        common: DashboardCommonContext::build(&db, auth.0),
        config: &CONFIG_CONTEXT,
        has_trackers: tracker_count_result.unwrap_or(0) > 0,
        commission_percent: (commission_value * 100.0) as u8
    })
}

#[get("/dashboard/trackers")]
fn get_dashboard_trackers(auth: AuthGuard, db: DbConn) -> Template {
    get_dashboard_trackers_args(auth, db, DashboardArgs {
        result: None
    })
}

#[get("/dashboard/trackers?<args>")]
fn get_dashboard_trackers_args(auth: AuthGuard, db: DbConn, args: DashboardArgs) -> Template {
    let mut trackers = Vec::new();

    tracker
        .filter(tracker_account_id.eq(auth.0))
        .order(tracker_label.asc())
        .load::<Tracker>(&*db)
        .map(|results| {
            for result in results {
                log::debug!("got tracker: {:?}", result);

                let total_earned: Option<f32> = balance
                    .filter(balance_account_id.eq(auth.0))
                    .filter(balance_tracker_id.eq(&result.id))
                    .select(sum(balance_amount))
                    .first(&*db)
                    .ok()
                    .and_then(|value: Option<BigDecimal>| {
                        if let Some(value_inner) = value {
                            value_inner.to_f32()
                        } else {
                            None
                        }
                    });

                trackers.push(DashboardTrackersContextTracker {
                    tracking_id: result.id,
                    label: result.label,
                    statistics_signups: result.statistics_signups.separated_string(),
                    statistics_paying: result.statistics_paying.separated_string(),
                    total_earned: total_earned.unwrap_or(0.0).separated_string_with_fixed_place(2)
                });
            }
        })
        .ok();

    Template::render("dashboard_trackers", &DashboardTrackersContext {
        create_failure: check_argument_value(&args.result, "create_failure"),
        create_success: check_argument_value(&args.result, "create_success"),
        remove_failure: check_argument_value(&args.result, "remove_failure"),
        remove_neutral: check_argument_value(&args.result, "remove_neutral"),
        remove_success: check_argument_value(&args.result, "remove_success"),
        trackers: trackers,
        common: DashboardCommonContext::build(&db, auth.0),
        config: &CONFIG_CONTEXT
    })
}

#[post("/dashboard/trackers/form/create", data = "<data>")]
fn post_dashboard_trackers_form_create(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardTrackersFormCreateData>
) -> Redirect {
    let data_inner = data.get();

    let now_date = Utc::now().naive_utc();
    let new_tracker_id = rand::thread_rng()
        .gen_ascii_chars()
        .take(10)
        .collect::<String>();

    let insert_result = diesel::insert_into(tracker)
        .values((
            &tracker_id.eq(&new_tracker_id),
            &tracker_label.eq(&data_inner.name),
            &tracker_account_id.eq(&auth.0),
            &tracker_created_at.eq(&now_date),
            &tracker_updated_at.eq(&now_date)
        ))
        .execute(&*db);

    log::debug!(
        "created tracker: {} named: {} for user_id: {}", new_tracker_id, data_inner.name, auth.0
    );

    Redirect::to(&format!("/dashboard/trackers/?result={}", if insert_result.is_ok() == true {
        "create_success"
    } else {
        "create_failure"
    }))
}

#[post("/dashboard/trackers/form/remove", data = "<data>")]
fn post_dashboard_trackers_form_remove(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardTrackersFormRemoveData>
) -> Redirect {
    let data_inner = data.get();

    let delete_result = diesel::delete(
        tracker
            .filter(tracker_account_id.eq(auth.0))
            .filter(tracker_id.eq_any(&data_inner.trackers))
    )
        .execute(&*db);

    let count_updated = delete_result.as_ref().unwrap_or(&0);

    log::debug!("removed {} tracker fields for user_id: {}", count_updated, auth.0);

    Redirect::to(&format!("/dashboard/trackers/?result={}", if count_updated > &0 {
        "remove_success"
    } else if delete_result.is_ok() == true {
        "remove_neutral"
    } else {
        "remove_failure"
    }))
}

#[get("/dashboard/payouts")]
fn get_dashboard_payouts(auth: AuthGuard, db: DbConn) -> Template {
    let mut payouts = Vec::new();
    let mut has_more = false;

    payout
        .filter(payout_account_id.eq(auth.0))
        .order(payout_created_at.asc())
        .limit(PAYOUTS_LIMIT_PER_PAGE + 1)
        .load::<Payout>(&*db)
        .map(|results| {
            for (index, result) in results.into_iter().enumerate() {
                if (index as i64) < PAYOUTS_LIMIT_PER_PAGE {
                    log::debug!("got payout #{}: {:?}", index, result);

                    let amount_value = result.amount
                        .to_f32()
                        .unwrap_or(0.0)
                        .separated_string_with_fixed_place(2);

                    payouts.push(DashboardPayoutsContextPayout {
                        number: result.number,
                        status: result.status,
                        amount: amount_value,
                        currency: result.currency,
                        account: result.account.unwrap_or("".to_string()),
                        invoice_url: result.invoice_url.unwrap_or("".to_string()),
                        date: result.created_at.date().format("%d/%m/%Y").to_string(),
                    });
                } else {
                    has_more = true;
                }
            }
        })
        .ok();

    Template::render("dashboard_payouts", &DashboardPayoutsContext {
        common: DashboardCommonContext::build(&db, auth.0),
        config: &CONFIG_CONTEXT,
        balance_total: get_balance(&db, auth.0, None),
        payouts: payouts,
        has_more: has_more
    })
}

#[get("/dashboard/account")]
fn get_dashboard_account(auth: AuthGuard, db: DbConn) -> Result<Template, Failure> {
    get_dashboard_account_args(auth, db, DashboardArgs {
        result: None
    })
}

#[get("/dashboard/account?<args>")]
fn get_dashboard_account_args(
    auth: AuthGuard,
    db: DbConn,
    args: DashboardArgs
) -> Result<Template, Failure> {
    let account_result = account
        .filter(account_id.eq(auth.0))
        .first::<Account>(&*db);

    if let Ok(account_inner) = account_result {
        let country_list = countries()
            .into_iter()
            .map(|country| (country.alpha2, country.name))
            .collect();

        Ok(Template::render("dashboard_account", &DashboardAccountContext {
            failure: check_argument_value(&args.result, "failure"),
            neutral: check_argument_value(&args.result, "neutral"),
            success: check_argument_value(&args.result, "success"),
            common: DashboardCommonContext::build(&db, auth.0),
            config: &CONFIG_CONTEXT,
            account: DashboardAccountContextAccount {
                email: account_inner.email
            },
            payout_methods: ACCOUNT_PAYOUT_METHODS,
            countries: country_list,
            payout: DashboardAccountContextPayout {
                full_name: account_inner.full_name.unwrap_or("".to_string()),
                address: account_inner.address.unwrap_or("".to_string()),
                country: account_inner.country.unwrap_or("".to_string()),
                method: account_inner.payout_method.unwrap_or("".to_string()),
                instructions: account_inner.payout_instructions.unwrap_or("".to_string())
            }
        }))
    } else {
        Err(Failure(Status::PreconditionFailed))
    }
}

#[post("/dashboard/account/form/account", data = "<data>")]
fn post_dashboard_account_form_account(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardAccountFormAccountData>
) -> Redirect {
    let data_inner = data.get();

    let update_result = if data_inner.password.is_empty() == false {
        diesel::update(account.filter(account_id.eq(auth.0)))
            .set((
                account_email.eq(&data_inner.email),
                account_password.eq(&auth_password_encode(&data_inner.password))
            ))
            .execute(&*db)
    } else {
        diesel::update(account.filter(account_id.eq(auth.0)))
            .set(account_email.eq(&data_inner.email))
            .execute(&*db)
    };

    let count_updated = update_result.as_ref().unwrap_or(&0);

    log::debug!("updated {} account base fields for user_id: {}", count_updated, auth.0);

    Redirect::to(&format!("/dashboard/account/?result={}", if count_updated > &0 {
        "success"
    } else if update_result.is_ok() == true {
        "neutral"
    } else {
        "failure"
    }))
}

#[post("/dashboard/account/form/payout", data = "<data>")]
fn post_dashboard_account_form_payout(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardAccountFormPayoutData>
) -> Redirect {
    let data_inner = data.get();

    let update_result = diesel::update(account.filter(account_id.eq(auth.0)))
        .set((
            account_full_name.eq(&data_inner.full_name),
            account_address.eq(&data_inner.address),
            account_country.eq(&data_inner.country),
            account_payout_method.eq(&data_inner.payout_method),
            account_payout_instructions.eq(&data_inner.payout_instructions)
        ))
        .execute(&*db);

    let count_updated = update_result.as_ref().unwrap_or(&0);

    log::debug!("updated {} account payout fields for user_id: {}", count_updated, auth.0);

    Redirect::to(&format!("/dashboard/account/?result={}", if count_updated > &0 {
        "success"
    } else if update_result.is_ok() == true {
        "neutral"
    } else {
        "failure"
    }))
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

#[get("/assets/javascripts/<file..>")]
fn get_assets_javascripts(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./javascripts").join(file)).ok()
}
