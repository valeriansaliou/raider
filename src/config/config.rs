// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::path::PathBuf;

use url_serde::SerdeUrl;

use super::defaults;

#[derive(Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub database: ConfigDatabase,
    pub email: ConfigEmail,
    pub assets: ConfigAssets,
    pub branding: ConfigBranding,
    pub tracker: ConfigTracker,
    pub payout: ConfigPayout,
}

#[derive(Deserialize)]
pub struct ConfigServer {
    #[serde(default = "defaults::server_log_level")]
    pub log_level: String,

    #[serde(default = "defaults::server_inet")]
    pub inet: SocketAddr,

    #[serde(default = "defaults::server_workers")]
    pub workers: u16,

    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct ConfigDatabase {
    pub url: SerdeUrl,

    #[serde(default = "defaults::database_pool_size")]
    pub pool_size: u32,

    #[serde(default = "defaults::database_idle_timeout")]
    pub idle_timeout: u64,

    #[serde(default = "defaults::database_connection_timeout")]
    pub connection_timeout: u64,

    pub password_salt: String,
}

#[derive(Deserialize)]
pub struct ConfigEmail {
    pub from: String,

    #[serde(default = "defaults::email_smtp_host")]
    pub smtp_host: String,

    #[serde(default = "defaults::email_smtp_port")]
    pub smtp_port: u16,

    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,

    #[serde(default = "defaults::email_smtp_encrypt")]
    pub smtp_encrypt: bool,
}

#[derive(Deserialize)]
pub struct ConfigAssets {
    #[serde(default = "defaults::assets_path")]
    pub path: PathBuf,
}

#[derive(Deserialize)]
pub struct ConfigBranding {
    #[serde(default = "defaults::branding_page_title")]
    pub page_title: String,

    pub page_url: SerdeUrl,
    pub help_url: SerdeUrl,
    pub support_url: SerdeUrl,
    pub icon_color: String,
    pub icon_url: SerdeUrl,
    pub logo_white_url: SerdeUrl,
    pub logo_dark_url: SerdeUrl,
}

#[derive(Deserialize)]
pub struct ConfigTracker {
    pub track_url: String,

    #[serde(default = "defaults::tracker_track_parameter")]
    pub track_parameter: String,

    #[serde(default = "defaults::tracker_commission_default")]
    pub commission_default: f32,
}

#[derive(Deserialize)]
pub struct ConfigPayout {
    #[serde(default = "defaults::payout_currency")]
    pub currency: String,
}
