// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::path::PathBuf;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::Cookies;

use super::asset_file::AssetFile;
use super::auth_guard::{AuthGuard, AuthAnonymousGuard, cleanup as auth_cleanup};
use APP_CONF;

#[derive(FromForm)]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(FromForm)]
pub struct SignupData {
    email: String,
    password: String,
}

#[derive(FromForm)]
pub struct RecoverData {
    email: String,
}

#[get("/")]
fn get_index(_anon: AuthAnonymousGuard) -> Redirect {
    Redirect::found("/initiate/")
}

#[get("/initiate")]
fn get_initiate_base(_anon: AuthAnonymousGuard) -> Redirect {
    Redirect::found("/initiate/login/")
}

#[get("/initiate/login")]
fn get_initiate_login(_anon: AuthAnonymousGuard) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./templates/initiate_login.html")).ok()
}

#[post("/initiate/login", data = "<data>")]
fn post_initiate_login(_anon: AuthAnonymousGuard, data: Form<LoginData>) -> Option<String> {
    // TODO

    Some("OK".to_string())
}

#[get("/initiate/signup")]
fn get_initiate_signup(_anon: AuthAnonymousGuard) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./templates/initiate_signup.html")).ok()
}

#[post("/initiate/signup", data = "<data>")]
fn post_initiate_signup(_anon: AuthAnonymousGuard, data: Form<SignupData>) -> Option<String> {
    // TODO

    Some("OK".to_string())
}

#[get("/initiate/recover")]
fn get_initiate_recover(_anon: AuthAnonymousGuard) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./templates/initiate_recover.html")).ok()
}

#[post("/initiate/recover", data = "<data>")]
fn post_initiate_recover(_anon: AuthAnonymousGuard, data: Form<RecoverData>) -> Option<String> {
    // TODO

    Some("OK".to_string())
}

#[get("/initiate/logout")]
fn get_initiate_logout(_auth: AuthGuard, cookies: Cookies) -> Redirect {
    auth_cleanup(cookies);

    Redirect::to("/initiate/")
}

#[get("/dashboard")]
fn get_dashboard_base(_auth: AuthGuard) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./templates/dashboard_base.html")).ok()
}

#[get("/dashboard/trackers")]
fn get_dashboard_trackers(_auth: AuthGuard) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./templates/dashboard_trackers.html")).ok()
}

#[get("/dashboard/payouts")]
fn get_dashboard_payouts(_auth: AuthGuard) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./templates/dashboard_payouts.html")).ok()
}

#[get("/dashboard/account")]
fn get_dashboard_account(_auth: AuthGuard) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./templates/dashboard_account.html")).ok()
}

#[get("/robots.txt")]
fn get_robots() -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./public/robots.txt")).ok()
}

#[get("/assets/fonts/<file..>")]
fn get_assets_fonts(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./fonts").join(file)).ok()
}

#[get("/assets/images/<file..>")]
fn get_assets_images(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./images").join(file)).ok()
}

#[get("/assets/stylesheets/<file..>")]
fn get_assets_stylesheets(file: PathBuf) -> Option<AssetFile> {
    AssetFile::open(APP_CONF.assets.path.join("./stylesheets").join(file)).ok()
}
