// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::path::PathBuf;

use super::config::ConfigTrackerBanner;

pub fn server_log_level() -> String {
    "warn".to_string()
}

pub fn server_inet() -> SocketAddr {
    "[::1]:8080".parse().unwrap()
}

pub fn server_workers() -> u16 {
    4
}

pub fn database_pool_size() -> u32 {
    4
}

pub fn database_idle_timeout() -> u64 {
    300
}

pub fn database_connection_timeout() -> u64 {
    10
}

pub fn email_smtp_host() -> String {
    "localhost".to_string()
}

pub fn email_smtp_port() -> u16 {
    587
}

pub fn email_smtp_encrypt() -> bool {
    true
}

pub fn assets_path() -> PathBuf {
    PathBuf::from("./res/assets/")
}

pub fn branding_page_title() -> String {
    "Affiliates".to_string()
}

pub fn tracker_track_parameter() -> String {
    "t".to_string()
}

pub fn tracker_commission_default() -> f32 {
    0.20
}

pub fn tracker_banner() -> Vec<ConfigTrackerBanner> {
    Vec::new()
}

pub fn payout_currency() -> String {
    "USD".to_string()
}
