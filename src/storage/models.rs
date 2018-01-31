// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use chrono::naive::NaiveDateTime;
use bigdecimal::BigDecimal;

use super::schemas::{account, balance, payout, tracker};

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "account"]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: Vec<u8>,
    pub recovery: Option<Vec<u8>>,
    pub commission: BigDecimal,
    pub full_name: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub payout_method: Option<String>,
    pub payout_instructions: Option<String>,
    pub notify_balance: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "balance"]
pub struct Balance {
    pub id: i32,
    pub amount: BigDecimal,
    pub currency: String,
    pub released: bool,
    pub trace: Option<String>,
    pub account_id: i32,
    pub tracker_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "payout"]
pub struct Payout {
    pub id: i32,
    pub number: i32,
    pub amount: BigDecimal,
    pub currency: String,
    pub status: String,
    pub account: Option<String>,
    pub invoice_url: Option<String>,
    pub account_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "tracker"]
pub struct Tracker {
    pub id: String,
    pub label: String,
    pub statistics_signups: i32,
    pub statistics_paying: i32,
    pub account_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "account"]
pub struct AccountRecoveryUpdate {
    pub recovery: Vec<u8>,
}
