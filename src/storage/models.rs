// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use chrono::naive::NaiveDateTime;
use uuid::Uuid;

use super::schemas::account;

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "account"]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
