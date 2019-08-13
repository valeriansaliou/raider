// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use bigdecimal::BigDecimal;
use chrono::offset::Utc;
use diesel;
use diesel::dsl::{count, max, sum};
use diesel::prelude::*;
use iso_country::data::all as countries;
use log;
use num_traits::cast::ToPrimitive;
use rand::{self, Rng};
use rocket::http::{Cookies, Status};
use rocket::request::{Form, FormItems, FromForm, FromFormValue};
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use separator::{FixedPlaceSeparatable, Separatable};
use std::collections::HashSet;
use std::path::PathBuf;
use validate::rules::email as validate_email;

use super::asset_file::AssetFile;
use super::auth_guard::{
    cleanup as auth_cleanup, insert as auth_insert, password_encode as auth_password_encode,
    password_policy_check as auth_password_policy_check, password_verify as auth_password_verify,
    recovery_generate as auth_recovery_generate, AuthAnonymousGuard, AuthGuard,
};
use super::context::{ConfigContext, CONFIG_CONTEXT};
use super::track_guard::TrackGuard;
use super::utilities::{
    check_argument_value, get_balance, get_balance_string, list_payouts, send_payout_emails,
};
use notifier::email::EmailNotifier;
use storage::choices::ACCOUNT_PAYOUT_METHODS;
use storage::db::DbConn;
use storage::models::{Account, AccountRecoveryUpdate, Tracker};
use storage::schemas::account::dsl::{
    account, address as account_address, commission as account_commission,
    country as account_country, created_at as account_created_at, email as account_email,
    full_name as account_full_name, id as account_id, notify_balance as account_notify_balance,
    password as account_password, payout_instructions as account_payout_instructions,
    payout_method as account_payout_method, recovery as account_recovery,
    updated_at as account_updated_at,
};
use storage::schemas::balance::dsl::{
    account_id as balance_account_id, amount as balance_amount, balance,
    released as balance_released, tracker_id as balance_tracker_id,
    updated_at as balance_updated_at,
};
use storage::schemas::payout::dsl::{
    account_id as payout_account_id, amount as payout_amount, created_at as payout_created_at,
    currency as payout_currency, id as payout_id, number as payout_number, payout,
    updated_at as payout_updated_at,
};
use storage::schemas::tracker::dsl::{
    account_id as tracker_account_id, created_at as tracker_created_at, id as tracker_id,
    label as tracker_label, tracker, updated_at as tracker_updated_at,
};
use track::payment::{
    handle_payment as track_handle_payment, handle_signup as track_handle_signup,
    run_notify_payment as track_run_notify_payment, HandlePaymentError as TrackHandlePaymentError,
    HandleSignupError as TrackHandleSignupError,
};
use APP_CONF;

#[derive(FromForm)]
pub struct InitiateArgs {
    result: Option<String>,
}

#[derive(FromForm)]
pub struct DashboardArgs {
    result: Option<String>,
}

#[derive(Deserialize)]
pub struct TrackPaymentData {
    amount: f32,
    currency: String,
    trace: Option<String>,
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
    name: String,
}

pub struct DashboardTrackersFormRemoveData {
    trackers: HashSet<String>,
}

#[derive(FromForm)]
pub struct DashboardAccountFormAccountData {
    email: String,
    password: String,
    notify_balance: Option<String>,
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
    pub config: &'a ConfigContext,
}

#[derive(Serialize)]
pub struct SignupContext<'a> {
    pub failure: bool,
    pub config: &'a ConfigContext,
}

#[derive(Serialize)]
pub struct RecoverContext<'a> {
    pub success: bool,
    pub failure: bool,
    pub config: &'a ConfigContext,
}

#[derive(Serialize)]
pub struct DashboardCommonContext {
    pub balance_pending: String,
}

#[derive(Serialize)]
pub struct DashboardBaseContext<'a> {
    pub common: DashboardCommonContext,
    pub config: &'a ConfigContext,
    pub has_trackers: bool,
    pub commission_percent: u8,
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
    pub config: &'a ConfigContext,
}

#[derive(Serialize)]
pub struct DashboardTrackersContextTracker {
    pub tracking_id: String,
    pub label: String,
    pub statistics_signups: String,
    pub total_earned: String,
}

#[derive(Serialize)]
pub struct DashboardPayoutsContext<'a> {
    pub request_success: bool,
    pub request_failure: bool,
    pub amount_failure: bool,
    pub amount_neutral: bool,
    pub config_failure: bool,
    pub common: DashboardCommonContext,
    pub config: &'a ConfigContext,
    pub balance_total: String,
    pub payouts_total: i64,
    pub payouts: Vec<DashboardPayoutsContextPayout>,
    pub has_more: bool,
}

#[derive(Serialize)]
pub struct DashboardPayoutsPartialPayoutsContext {
    pub payouts: Vec<DashboardPayoutsContextPayout>,
    pub has_more: bool,
}

#[derive(Serialize)]
pub struct DashboardPayoutsContextPayout {
    pub number: i32,
    pub status: String,
    pub amount: String,
    pub currency: String,
    pub account: String,
    pub invoice_url: String,
    pub date: String,
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
    pub payout: DashboardAccountContextPayout,
}

#[derive(Serialize)]
pub struct DashboardAccountContextAccount {
    pub email: String,
    pub notify_balance: bool,
}

#[derive(Serialize)]
pub struct DashboardAccountContextPayout {
    pub full_name: String,
    pub address: String,
    pub country: String,
    pub method: String,
    pub instructions: String,
}

const TRACKERS_PER_ACCOUNT_MAXIMUM: i64 = 100;

impl DashboardCommonContext {
    fn build(db: &DbConn, user_id: i32) -> DashboardCommonContext {
        DashboardCommonContext {
            balance_pending: get_balance_string(db, user_id, Some(false)),
        }
    }
}

impl<'f> FromForm<'f> for DashboardTrackersFormRemoveData {
    type Error = ();

    fn from_form(form_items: &mut FormItems<'f>, _: bool) -> Result<Self, Self::Error> {
        let mut update = DashboardTrackersFormRemoveData {
            trackers: HashSet::new(),
        };

        for form_item in form_items {
            let value = String::from_form_value(form_item.value).or(Err(()))?;

            match form_item.key.as_str() {
                "tracker" => update.trackers.insert(value),
                _ => {
                    return Err(());
                }
            };
        }

        Ok(update)
    }
}

#[get("/")]
pub fn get_index(_anon: AuthAnonymousGuard) -> Redirect {
    Redirect::found("/initiate/")
}

#[get("/initiate")]
pub fn get_initiate_base(_anon: AuthAnonymousGuard) -> Redirect {
    Redirect::found("/initiate/login/")
}

#[get("/initiate/login?<args..>")]
pub fn get_initiate_login(_anon: AuthAnonymousGuard, args: Form<InitiateArgs>) -> Template {
    Template::render(
        "initiate_login",
        &LoginContext {
            failure: check_argument_value(&args.result, "failure"),
            config: &CONFIG_CONTEXT,
        },
    )
}

#[post("/initiate/login/form/login", data = "<data>")]
pub fn post_initiate_login_form_login(
    _anon: AuthAnonymousGuard,
    cookies: Cookies,
    db: DbConn,
    data: Form<LoginData>,
) -> Redirect {
    if data.email.is_empty() == false
        && data.password.is_empty() == false
        && validate_email().validate(&data.email).is_ok() == true
        && auth_password_policy_check(&data.password) == true
    {
        let account_result = account
            .filter(account_email.eq(&data.email))
            .first::<Account>(&*db);

        match account_result {
            Ok(result) => {
                let mut is_auth_valid = auth_password_verify(&result.password, &data.password);

                // Attempt to check recovery password?
                if is_auth_valid == false {
                    if let Some(ref recovery) = result.recovery {
                        is_auth_valid = auth_password_verify(recovery, &data.password);
                    }
                }

                // Password is valid?
                if is_auth_valid == true {
                    // Erase any stored recovery password?
                    if result.recovery.is_some() == true {
                        let recovery_update =
                            diesel::update(account.filter(account_id.eq(result.id)))
                                .set(&AccountRecoveryUpdate {
                                    recovery: Vec::new(),
                                })
                                .execute(&*db);

                        match recovery_update {
                            Ok(_) => log::info!("cleared recovery password"),
                            Err(err) => log::error!("failed clearing recovery password: {}", err),
                        }
                    }

                    // Log-in user (set cookie)
                    auth_insert(cookies, result.id.to_string());

                    return Redirect::to("/dashboard/");
                }
            }
            Err(err) => {
                log::debug!("account not retrieved for login: {}", err);
            }
        };
    }

    Redirect::to("/initiate/login/?result=failure")
}

#[get("/initiate/signup?<args..>")]
pub fn get_initiate_signup(_anon: AuthAnonymousGuard, args: Form<InitiateArgs>) -> Template {
    Template::render(
        "initiate_signup",
        &SignupContext {
            failure: check_argument_value(&args.result, "failure"),
            config: &CONFIG_CONTEXT,
        },
    )
}

#[post("/initiate/signup/form/signup", data = "<data>")]
pub fn post_initiate_signup_form_signup(
    _anon: AuthAnonymousGuard,
    cookies: Cookies,
    db: DbConn,
    data: Form<SignupData>,
) -> Redirect {
    if data.email.is_empty() == false
        && data.password.is_empty() == false
        && validate_email().validate(&data.email).is_ok() == true
        && data.password == data.password_repeat
        && auth_password_policy_check(&data.password) == true
    {
        let now_date = Utc::now().naive_utc();

        let insert_result = diesel::insert_into(account)
            .values((
                &account_email.eq(&data.email),
                &account_password.eq(&auth_password_encode(&data.password)),
                &account_commission.eq(BigDecimal::from(APP_CONF.tracker.commission_default)),
                &account_created_at.eq(&now_date),
                &account_updated_at.eq(&now_date),
            ))
            .execute(&*db);

        if insert_result.is_ok() == true {
            let account_result = account
                .filter(account_email.eq(&data.email))
                .first::<Account>(&*db);

            match account_result {
                Ok(result) => {
                    // Log-in user (set cookie)
                    auth_insert(cookies, result.id.to_string());

                    return Redirect::to("/dashboard/");
                }
                Err(err) => {
                    log::debug!("account not retrieved for login: {}", err);
                }
            };
        }
    }

    Redirect::to("/initiate/signup/?result=failure")
}

#[get("/initiate/recover?<args..>")]
pub fn get_initiate_recover(_anon: AuthAnonymousGuard, args: Form<InitiateArgs>) -> Template {
    Template::render(
        "initiate_recover",
        &RecoverContext {
            failure: check_argument_value(&args.result, "failure"),
            success: check_argument_value(&args.result, "success"),
            config: &CONFIG_CONTEXT,
        },
    )
}

#[post("/initiate/recover/form/recover", data = "<data>")]
pub fn post_initiate_recover_form_recover(
    _anon: AuthAnonymousGuard,
    db: DbConn,
    data: Form<RecoverData>,
) -> Redirect {
    if data.email.is_empty() == false && validate_email().validate(&data.email).is_ok() == true {
        let result = account
            .filter(account_email.eq(&data.email))
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
                    "Your account main password was not changed. Please update it once logged in.",
                );

                // Send password recovery email
                if EmailNotifier::dispatch(
                    &account_result.email,
                    "Recover your password".to_string(),
                    &message,
                )
                .is_ok()
                    == true
                {
                    return Redirect::to("/initiate/recover/?result=success");
                }
            }
        }
    }

    Redirect::to("/initiate/recover/?result=failure")
}

#[get("/initiate/logout")]
pub fn get_initiate_logout(_auth: AuthGuard, cookies: Cookies) -> Redirect {
    auth_cleanup(cookies);

    Redirect::to("/initiate/")
}

#[get("/dashboard")]
pub fn get_dashboard_base(auth: AuthGuard, db: DbConn) -> Redirect {
    let tracker_count_result = tracker
        .filter(tracker_account_id.eq(auth.0))
        .select(count(tracker_id))
        .first(&*db);

    if tracker_count_result.unwrap_or(0) > 0 {
        Redirect::to("/dashboard/trackers/")
    } else {
        Redirect::to("/dashboard/welcome/")
    }
}

#[get("/dashboard/welcome")]
pub fn get_dashboard_welcome(auth: AuthGuard, db: DbConn) -> Template {
    let account_result = account.filter(account_id.eq(auth.0)).first::<Account>(&*db);

    let tracker_count_result = tracker
        .filter(tracker_account_id.eq(auth.0))
        .select(count(tracker_id))
        .first(&*db);

    let commission_value = if let Ok(account_inner) = account_result {
        account_inner.commission.to_f32().unwrap_or(0.00)
    } else {
        0.00
    };

    Template::render(
        "dashboard_welcome",
        &DashboardBaseContext {
            common: DashboardCommonContext::build(&db, auth.0),
            config: &CONFIG_CONTEXT,
            has_trackers: tracker_count_result.unwrap_or(0) > 0,
            commission_percent: (commission_value * 100.0) as u8,
        },
    )
}

#[get("/dashboard/trackers?<args..>")]
pub fn get_dashboard_trackers(auth: AuthGuard, db: DbConn, args: Form<DashboardArgs>) -> Template {
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
                    total_earned: total_earned
                        .unwrap_or(0.0)
                        .separated_string_with_fixed_place(2),
                });
            }
        })
        .ok();

    Template::render(
        "dashboard_trackers",
        &DashboardTrackersContext {
            create_failure: check_argument_value(&args.result, "create_failure"),
            create_success: check_argument_value(&args.result, "create_success"),
            remove_failure: check_argument_value(&args.result, "remove_failure"),
            remove_neutral: check_argument_value(&args.result, "remove_neutral"),
            remove_success: check_argument_value(&args.result, "remove_success"),
            trackers: trackers,
            common: DashboardCommonContext::build(&db, auth.0),
            config: &CONFIG_CONTEXT,
        },
    )
}

#[post("/dashboard/trackers/form/create", data = "<data>")]
pub fn post_dashboard_trackers_form_create(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardTrackersFormCreateData>,
) -> Redirect {
    let now_date = Utc::now().naive_utc();
    let new_tracker_id = rand::thread_rng()
        .gen_ascii_chars()
        .take(10)
        .collect::<String>();

    let trackers_total = tracker
        .filter(tracker_account_id.eq(auth.0))
        .select(count(tracker_id))
        .first(&*db)
        .unwrap_or(0);

    let insert_result = if trackers_total < TRACKERS_PER_ACCOUNT_MAXIMUM {
        let result = diesel::insert_into(tracker)
            .values((
                &tracker_id.eq(&new_tracker_id),
                &tracker_label.eq(&data.name),
                &tracker_account_id.eq(&auth.0),
                &tracker_created_at.eq(&now_date),
                &tracker_updated_at.eq(&now_date),
            ))
            .execute(&*db)
            .or(Err(()));

        log::debug!(
            "created tracker: {} named: {} for user_id: {}",
            new_tracker_id,
            data.name,
            auth.0
        );

        result
    } else {
        Err(())
    };

    Redirect::to(format!(
        "/dashboard/trackers/?result={}",
        if insert_result.is_ok() == true {
            "create_success"
        } else {
            "create_failure"
        }
    ))
}

#[post("/dashboard/trackers/form/remove", data = "<data>")]
pub fn post_dashboard_trackers_form_remove(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardTrackersFormRemoveData>,
) -> Redirect {
    let delete_result = diesel::delete(
        tracker
            .filter(tracker_account_id.eq(auth.0))
            .filter(tracker_id.eq_any(&data.trackers)),
    )
    .execute(&*db);

    let count_updated = delete_result.as_ref().unwrap_or(&0);

    log::debug!(
        "removed {} tracker fields for user_id: {}",
        count_updated,
        auth.0
    );

    Redirect::to(format!(
        "/dashboard/trackers/?result={}",
        if count_updated > &0 {
            "remove_success"
        } else if delete_result.is_ok() == true {
            "remove_neutral"
        } else {
            "remove_failure"
        }
    ))
}

#[get("/dashboard/payouts?<args..>")]
pub fn get_dashboard_payouts(auth: AuthGuard, db: DbConn, args: Form<DashboardArgs>) -> Template {
    let payouts_total = payout
        .filter(payout_account_id.eq(auth.0))
        .select(count(payout_id))
        .first(&*db)
        .unwrap_or(0);

    let (payouts, has_more) = list_payouts(&db, auth.0, 1);

    Template::render(
        "dashboard_payouts",
        &DashboardPayoutsContext {
            request_success: check_argument_value(&args.result, "request_success"),
            request_failure: check_argument_value(&args.result, "request_failure"),
            amount_failure: check_argument_value(&args.result, "amount_failure"),
            amount_neutral: check_argument_value(&args.result, "amount_neutral"),
            config_failure: check_argument_value(&args.result, "config_failure"),
            common: DashboardCommonContext::build(&db, auth.0),
            config: &CONFIG_CONTEXT,
            balance_total: get_balance_string(&db, auth.0, None),
            payouts_total: payouts_total,
            payouts: payouts,
            has_more: has_more,
        },
    )
}

#[get("/dashboard/payouts/partial/payouts/<page_number>")]
pub fn get_dashboard_payouts_partial_payouts(
    auth: AuthGuard,
    db: DbConn,
    page_number: u16,
) -> Template {
    let (payouts, has_more) = list_payouts(&db, auth.0, page_number);

    Template::render(
        "dashboard_payouts_partial_payouts",
        &DashboardPayoutsPartialPayoutsContext {
            payouts: payouts,
            has_more: has_more,
        },
    )
}

#[post("/dashboard/payouts/form/request")]
pub fn post_dashboard_payouts_form_request(
    auth: AuthGuard,
    db: DbConn,
) -> Result<Redirect, Status> {
    let account_result = account.filter(account_id.eq(auth.0)).first::<Account>(&*db);

    if let Ok(account_inner) = account_result {
        let result_code = {
            // Check if user has all payout details properly configured
            if account_inner.full_name.unwrap_or_default().is_empty()
                || account_inner.address.unwrap_or_default().is_empty()
                || account_inner.country.unwrap_or_default().is_empty()
                || account_inner.payout_method.unwrap_or_default().is_empty()
                || account_inner
                    .payout_instructions
                    .unwrap_or_default()
                    .is_empty()
            {
                "config_failure"
            } else {
                // Check if there is money due
                let balance_due = get_balance(&db, auth.0, Some(false));

                if balance_due > 0.0 {
                    if balance_due >= APP_CONF.payout.amount_minimum {
                        let now_date = Utc::now().naive_utc();

                        // Bump all balance contents to mark them as requested
                        let update_result = diesel::update(
                            balance
                                .filter(balance_account_id.eq(auth.0))
                                .filter(balance_released.eq(false)),
                        )
                        .set((balance_released.eq(true), balance_updated_at.eq(&now_date)))
                        .execute(&*db);

                        // Acquire latest payout number
                        let maximum_result = payout
                            .filter(payout_account_id.eq(auth.0))
                            .select(max(payout_number))
                            .first::<Option<i32>>(&*db)
                            .map(|value| if value.is_none() { Some(0) } else { value });

                        match (update_result, maximum_result) {
                            (Ok(_), Ok(Some(maximum_number))) => {
                                // Create payout
                                let insert_result = diesel::insert_into(payout)
                                    .values((
                                        &payout_amount.eq(BigDecimal::from(balance_due)),
                                        &payout_number.eq(maximum_number + 1),
                                        &payout_currency.eq(&APP_CONF.payout.currency),
                                        &payout_account_id.eq(auth.0),
                                        &payout_created_at.eq(&now_date),
                                        &payout_updated_at.eq(&now_date),
                                    ))
                                    .execute(&*db);

                                if insert_result.is_ok() == true {
                                    send_payout_emails(
                                        auth.0,
                                        &account_inner.email,
                                        balance_due,
                                        &APP_CONF.payout.currency,
                                    );

                                    "request_success"
                                } else {
                                    "request_failure"
                                }
                            }
                            _ => "request_failure",
                        }
                    } else {
                        "amount_failure"
                    }
                } else {
                    "amount_neutral"
                }
            }
        };

        Ok(Redirect::to(format!(
            "/dashboard/payouts/?result={}",
            result_code
        )))
    } else {
        Err(Status::PreconditionFailed)
    }
}

#[get("/dashboard/account?<args..>")]
pub fn get_dashboard_account(
    auth: AuthGuard,
    db: DbConn,
    args: Form<DashboardArgs>,
) -> Result<Template, Status> {
    let account_result = account.filter(account_id.eq(auth.0)).first::<Account>(&*db);

    if let Ok(account_inner) = account_result {
        let country_list = countries()
            .into_iter()
            .map(|country| (country.alpha2, country.name))
            .collect();

        Ok(Template::render(
            "dashboard_account",
            &DashboardAccountContext {
                failure: check_argument_value(&args.result, "failure"),
                neutral: check_argument_value(&args.result, "neutral"),
                success: check_argument_value(&args.result, "success"),
                common: DashboardCommonContext::build(&db, auth.0),
                config: &CONFIG_CONTEXT,
                account: DashboardAccountContextAccount {
                    email: account_inner.email,
                    notify_balance: account_inner.notify_balance,
                },
                payout_methods: ACCOUNT_PAYOUT_METHODS,
                countries: country_list,
                payout: DashboardAccountContextPayout {
                    full_name: account_inner.full_name.unwrap_or("".to_string()),
                    address: account_inner.address.unwrap_or("".to_string()),
                    country: account_inner.country.unwrap_or("".to_string()),
                    method: account_inner.payout_method.unwrap_or("".to_string()),
                    instructions: account_inner.payout_instructions.unwrap_or("".to_string()),
                },
            },
        ))
    } else {
        Err(Status::PreconditionFailed)
    }
}

#[post("/dashboard/account/form/account", data = "<data>")]
pub fn post_dashboard_account_form_account(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardAccountFormAccountData>,
) -> Redirect {
    let notify_balance_value = data.notify_balance == Some("1".to_string());

    let update_result = if data.password.is_empty() == false {
        if auth_password_policy_check(&data.password) == true {
            diesel::update(account.filter(account_id.eq(auth.0)))
                .set((
                    account_email.eq(&data.email),
                    account_password.eq(&auth_password_encode(&data.password)),
                    account_notify_balance.eq(&notify_balance_value),
                ))
                .execute(&*db)
                .or(Err(()))
        } else {
            Err(())
        }
    } else {
        diesel::update(account.filter(account_id.eq(auth.0)))
            .set((
                account_email.eq(&data.email),
                account_notify_balance.eq(&notify_balance_value),
            ))
            .execute(&*db)
            .or(Err(()))
    };

    let count_updated = update_result.as_ref().unwrap_or(&0);

    log::debug!(
        "updated {} account base fields for user_id: {}",
        count_updated,
        auth.0
    );

    Redirect::to(format!(
        "/dashboard/account/?result={}",
        if count_updated > &0 {
            "success"
        } else if update_result.is_ok() == true {
            "neutral"
        } else {
            "failure"
        }
    ))
}

#[post("/dashboard/account/form/payout", data = "<data>")]
pub fn post_dashboard_account_form_payout(
    auth: AuthGuard,
    db: DbConn,
    data: Form<DashboardAccountFormPayoutData>,
) -> Redirect {
    let update_result = diesel::update(account.filter(account_id.eq(auth.0)))
        .set((
            account_full_name.eq(&data.full_name),
            account_address.eq(&data.address),
            account_country.eq(&data.country),
            account_payout_method.eq(&data.payout_method),
            account_payout_instructions.eq(&data.payout_instructions),
        ))
        .execute(&*db);

    let count_updated = update_result.as_ref().unwrap_or(&0);

    log::debug!(
        "updated {} account payout fields for user_id: {}",
        count_updated,
        auth.0
    );

    Redirect::to(format!(
        "/dashboard/account/?result={}",
        if count_updated > &0 {
            "success"
        } else if update_result.is_ok() == true {
            "neutral"
        } else {
            "failure"
        }
    ))
}

#[post(
    "/track/payment/<tracking_id>",
    data = "<data>",
    format = "application/json"
)]
pub fn post_track_payment(
    _auth: TrackGuard,
    db: DbConn,
    tracking_id: String,
    data: Json<TrackPaymentData>,
) -> Result<(), Status> {
    match track_handle_payment(
        &db,
        &tracking_id,
        data.amount,
        &data.currency.to_uppercase(),
        &data.trace,
    ) {
        Ok(results) => {
            if let Some((
                should_notify,
                email,
                source_tracker_id,
                commission_amount,
                commission_currency,
            )) = results
            {
                // Notify user about received commission
                if should_notify == true {
                    track_run_notify_payment(
                        email,
                        source_tracker_id,
                        commission_amount,
                        commission_currency,
                    );
                }
            }

            Ok(())
        }
        Err(TrackHandlePaymentError::InvalidAmount) => Err(Status::BadRequest),
        Err(TrackHandlePaymentError::BadCurrency) => Err(Status::PreconditionFailed),
        Err(TrackHandlePaymentError::NotFound) => Err(Status::NotFound),
    }
}

#[post("/track/signup/<tracking_id>")]
pub fn post_track_signup(_auth: TrackGuard, db: DbConn, tracking_id: String) -> Result<(), Status> {
    match track_handle_signup(&db, &tracking_id) {
        Ok(_) => Ok(()),
        Err(TrackHandleSignupError::NotFound) => Err(Status::NotFound),
    }
}

#[get("/robots.txt")]
pub fn get_robots() -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./public/robots.txt")).ok()
}

#[get("/assets/fonts/<file..>")]
pub fn get_assets_fonts(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./fonts").join(file)).ok()
}

#[get("/assets/images/<file..>")]
pub fn get_assets_images(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./images").join(file)).ok()
}

#[get("/assets/stylesheets/<file..>")]
pub fn get_assets_stylesheets(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./stylesheets").join(file)).ok()
}

#[get("/assets/javascripts/<file..>")]
pub fn get_assets_javascripts(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./javascripts").join(file)).ok()
}
