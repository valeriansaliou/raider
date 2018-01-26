// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use chrono::naive::NaiveDateTime;

use super::schemas::account;

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "account"]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: Vec<u8>,
    pub recovery: Option<Vec<u8>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "account"]
pub struct AccountRecoveryUpdate {
    pub recovery: Vec<u8>
}
