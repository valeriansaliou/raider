// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use bigdecimal::BigDecimal;
use diesel::dsl::sum;
use diesel::prelude::*;
use log;
use num_traits::cast::ToPrimitive;
use separator::FixedPlaceSeparatable;
use std::cmp;

use super::routes::DashboardPayoutsContextPayout;
use notifier::email::EmailNotifier;
use storage::db::DbConn;
use storage::models::Payout;
use storage::schemas::balance::dsl::{
    account_id as balance_account_id, amount as balance_amount, balance,
    released as balance_released,
};
use storage::schemas::payout::dsl::{
    account_id as payout_account_id, created_at as payout_created_at, payout,
};
use APP_CONF;

const PAYOUTS_LIMIT_PER_PAGE: i64 = 20;

pub fn get_balance(db: &DbConn, user_id: i32, released: Option<bool>) -> f32 {
    let balance_result = if let Some(released_inner) = released {
        balance
            .filter(balance_account_id.eq(user_id))
            .filter(balance_released.eq(released_inner))
            .select(sum(balance_amount))
            .first(&**db)
    } else {
        balance
            .filter(balance_account_id.eq(user_id))
            .select(sum(balance_amount))
            .first(&**db)
    };

    let balance_count: Option<f32> = balance_result.ok().and_then(|value: Option<BigDecimal>| {
        if let Some(value_inner) = value {
            value_inner.to_f32()
        } else {
            None
        }
    });

    balance_count.unwrap_or(0.0)
}

pub fn get_balance_string(db: &DbConn, user_id: i32, released: Option<bool>) -> String {
    get_balance(db, user_id, released).separated_string_with_fixed_place(2)
}

pub fn check_argument_value(argument: &Option<String>, against: &str) -> bool {
    if let &Some(ref value) = argument {
        value == against
    } else {
        false
    }
}

pub fn list_payouts(
    db: &DbConn,
    user_id: i32,
    page_number: u16,
) -> (Vec<DashboardPayoutsContextPayout>, bool) {
    let mut payouts = Vec::new();
    let mut has_more = false;

    payout
        .filter(payout_account_id.eq(user_id))
        .order(payout_created_at.desc())
        .limit(PAYOUTS_LIMIT_PER_PAGE + 1)
        .offset(paging_to_offset(page_number, PAYOUTS_LIMIT_PER_PAGE))
        .load::<Payout>(&**db)
        .map(|results| {
            for (index, result) in results.into_iter().enumerate() {
                if (index as i64) < PAYOUTS_LIMIT_PER_PAGE {
                    log::debug!("got payout #{}: {:?}", index, result);

                    let amount_value = result
                        .amount
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

    (payouts, has_more)
}

pub fn send_payout_emails(user_id: i32, user_email: &str, balance_due: f32, currency: &str) {
    // Send request email to administrator
    {
        // Generate message
        let mut message = String::new();

        message.push_str(&format!(
            "A payout of {} {} has been requested by user #{} with email: {}\n\n",
            currency,
            balance_due.separated_string_with_fixed_place(2),
            user_id,
            user_email
        ));

        message.push_str("Here are the steps to take:\n\n");
        message
            .push_str(" 1. Review the pending payout in the database and accept or refuse it.\n");
        message.push_str(" 2. Generate an invoice and update the database accordingly.\n");
        message.push_str(" 3. Send the money using user payout details.\n");
        message.push_str(" 4. Notify the user by email that the payout has been processed.\n");
        message.push_str(" 5. Mark the payout as processed in the database.");

        // Send email
        if EmailNotifier::dispatch(
            &APP_CONF.payout.administrator_email,
            "Pending payout request".to_string(),
            &message,
        )
        .is_ok()
            == true
        {
            log::debug!(
                "sent payout request email to administrator on: {}",
                &APP_CONF.payout.administrator_email
            );
        } else {
            log::error!(
                "could not send payout request email to administrator on: {}",
                &APP_CONF.payout.administrator_email
            );
        }
    }

    // Send confirmation email to user
    {
        // Generate message
        let mut message = String::new();

        message.push_str("Hi,\n\n");

        message.push_str(&format!(
            "Your payout request of {} {} has been submitted for processing.\n\n",
            currency,
            balance_due.separated_string_with_fixed_place(2)
        ));

        message.push_str("Our team has been notified and will process it as soon as possible. ");
        message.push_str("The money will then be sent to your registered payout method.");

        // Send email
        if EmailNotifier::dispatch(user_email, "Payout request submitted".to_string(), &message)
            .is_ok()
            == true
        {
            log::debug!("sent payout confirmation email to user on: {}", user_email);
        } else {
            log::error!(
                "could not send payout confirmation email to user on: {}",
                user_email
            );
        }
    }
}

fn paging_to_offset(page_number: u16, limit_per_page: i64) -> i64 {
    ((cmp::max(page_number, 1) - 1) as i64) * limit_per_page
}
