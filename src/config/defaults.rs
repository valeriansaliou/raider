// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::path::PathBuf;

pub fn server_log_level() -> String {
    "warn".to_string()
}

pub fn server_inet() -> SocketAddr {
    "[::1]:8080".parse().unwrap()
}

pub fn server_workers() -> u16 {
    4
}

pub fn assets_path() -> PathBuf {
    PathBuf::from("./res/assets/")
}
