// Raider
//
// Affiliates dashboard
// Copyright: 2021, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use bigdecimal::BigDecimal;
use chrono::offset::Utc;
use diesel;
use diesel::prelude::*;
use validate::rules::email as validate_email;

use notifier::email::EmailNotifier;
use responder::auth_guard::password_generate as auth_password_generate;
use storage::db::DbConn;
use storage::schemas::account::dsl::{
    account, address as account_address, commission as account_commission,
    country as account_country, created_at as account_created_at, email as account_email,
    full_name as account_full_name, password as account_password, updated_at as account_updated_at,
};
use APP_CONF;

pub enum HandleAccountError {
    Aborted,
    Duplicate,
    InvalidEmail,
}

pub fn handle_account(
    db: &DbConn,
    email: &str,
    full_name: &Option<String>,
    address: &Option<String>,
    country: &Option<String>,
) -> Result<(), HandleAccountError> {
    log::debug!("account management handle: {}", email);

    // Validate email address against policy
    if email.is_empty() == false && validate_email().validate(email).is_ok() == true {
        // Auto-generate a strong random password
        let password_params = auth_password_generate();

        // Insert account
        let now_date = Utc::now().naive_utc();

        let insert_result = diesel::insert_into(account)
            .values((
                &account_email.eq(email),
                &account_password.eq(&password_params.0),
                &account_full_name.eq(full_name),
                &account_address.eq(address),
                &account_country.eq(country),
                &account_commission.eq(BigDecimal::from(APP_CONF.tracker.commission_default)),
                &account_created_at.eq(&now_date),
                &account_updated_at.eq(&now_date),
            ))
            .execute(&**db);

        if insert_result.is_ok() == true {
            log::debug!(
                "will send created account password to email: {} with password: {}",
                email,
                password_params.1
            );

            // Generate account password message
            let mut message = String::new();

            message.push_str("Hi,\n\n");

            message.push_str(&format!(
                "Your {} account with email: {} has been created.\n",
                &APP_CONF.branding.page_title, email
            ));

            message.push_str(&format!(
                "Please login with this password to access your dashboard: {}\n\n",
                password_params.1
            ));

            message.push_str("You may change this password once logged in.");

            // Send account password email
            if EmailNotifier::dispatch(email, "Your account password".to_string(), &message).is_ok()
                == true
            {
                Ok(())
            } else {
                Err(HandleAccountError::Aborted)
            }
        } else {
            // Account is likely duplicate (as there was a database failure)
            log::warn!(
                "account: {} could not be created due to database rejection",
                email
            );

            Err(HandleAccountError::Duplicate)
        }
    } else {
        log::warn!(
            "account: {} could not be created due to invalid email",
            email
        );

        Err(HandleAccountError::InvalidEmail)
    }
}
