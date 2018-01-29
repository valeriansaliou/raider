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
        commission -> Numeric,
        full_name -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        payout_method -> Nullable<Varchar>,
        payout_instructions -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    balance (id) {
        id -> Integer,
        amount -> Numeric,
        currency -> VarChar,
        status -> VarChar,
        trace -> Nullable<Text>,
        account_id -> Integer,
        tracker_id -> Nullable<VarChar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    payout (id) {
        id -> Integer,
        number -> Integer,
        amount -> Numeric,
        currency -> VarChar,
        status -> VarChar,
        account -> Nullable<VarChar>,
        invoice_url -> Nullable<VarChar>,
        account_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tracker (id) {
        id -> VarChar,
        label -> VarChar,
        statistics_signups -> Integer,
        statistics_paying -> Integer,
        account_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(balance -> account(account_id));
joinable!(balance -> tracker(tracker_id));
joinable!(payout -> account(account_id));
joinable!(tracker -> account(account_id));
