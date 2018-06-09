// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::thread;
use std::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use log;
use reqwest::{Client, StatusCode};

use APP_CONF;

const POLL_RATE_SECONDS: u64 = 86400;
const RETRY_POLL_SECONDS: u64 = 30;

lazy_static! {
    static ref RATES: Arc<RwLock<HashMap<String, f32>>> = Arc::new(RwLock::new(HashMap::new()));

    static ref HTTP_CLIENT: Client = Client::builder()
        .timeout(Duration::from_secs(20))
        .gzip(true)
        .enable_hostname_verification()
        .build()
        .unwrap();
}

#[derive(Deserialize)]
struct FixerLatestResponse {
    rates: HashMap<String, f32>,
}

fn store_rates(rates: HashMap<String, f32>) {
    let mut store = RATES.write().unwrap();

    *store = rates;
}

fn update_rates() -> Result<(), ()> {
    log::debug!("acquiring updated exchange rates");

    // Acquire latest rates from Fixer.io
    let response = HTTP_CLIENT
        .get(&format!(
            "{}/api/latest?access_key={}&base={}",
            &APP_CONF.exchange.fixer.endpoint, &APP_CONF.exchange.fixer.access_key, &APP_CONF.payout.currency
        ))
        .send();

    if let Ok(mut response_inner) = response {
        let status = response_inner.status();

        log::debug!("received updated exchange rates");

        if status == StatusCode::Ok {
            if let Ok(response_json) = response_inner.json::<FixerLatestResponse>() {
                log::debug!("got updated exchange rates: {:?}", &response_json.rates);

                store_rates(response_json.rates);

                return Ok(());
            } else {
                log::error!("got invalid json when requesting updated exchange rates")
            }
        } else {
            log::error!("got bad status code when requesting updated exchange rates")
        }
    } else {
        log::error!("could not request updated exchange rates");
    }

    log::info!(
        "scheduled an exchange rates update retry in {} seconds",
        RETRY_POLL_SECONDS
    );

    // Re-schedule an update after a few seconds
    thread::sleep(Duration::from_secs(RETRY_POLL_SECONDS));

    return update_rates();
}

pub fn normalize(amount: f32, currency: &str) -> Result<f32, ()> {
    if currency == APP_CONF.payout.currency {
        Ok(amount)
    } else {
        if let Ok(ref store) = RATES.read() {
            if let Some(rate) = store.get(currency) {
                if rate > &0.0 {
                    Ok((1.0 / rate) * amount)
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

pub fn run() {
    loop {
        log::debug!("running an exchange poll operation...");

        update_rates().ok();

        log::info!("ran exchange poll operation");

        // Hold for next poll run
        thread::sleep(Duration::from_secs(POLL_RATE_SECONDS));
    }
}
