// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::path::PathBuf;

use super::defaults;
use prober::mode::Mode;

#[derive(Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub assets: ConfigAssets,
}

#[derive(Deserialize)]
pub struct ConfigServer {
    #[serde(default = "defaults::server_log_level")]
    pub log_level: String,

    #[serde(default = "defaults::server_inet")]
    pub inet: SocketAddr,

    #[serde(default = "defaults::server_workers")]
    pub workers: u16,
}

#[derive(Deserialize)]
pub struct ConfigAssets {
    #[serde(default = "defaults::assets_path")]
    pub path: PathBuf,
}
