// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::response::Redirect;

#[error(403)]
pub fn forbidden() -> Redirect {
    Redirect::to("/initiate/")
}

#[error(410)]
pub fn gone() -> Redirect {
    Redirect::to("/dashboard/")
}
