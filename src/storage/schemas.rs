// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

table! {
    account (id) {
        id -> Integer,
        email -> Varchar,
        password -> Binary,
        recovery -> Nullable<Binary>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
