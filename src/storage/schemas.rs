// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

table! {
    use diesel::types::*;
    use uuid::Uuid;

    account (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
