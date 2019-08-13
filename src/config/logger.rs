// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use log;
use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};

pub struct ConfigLogger;

impl log::Log for ConfigLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Debug
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("({}) - {}", record.level(), record.args());
        }
    }
}

impl ConfigLogger {
    pub fn init(level: LogLevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(level);
            Box::new(ConfigLogger)
        })
    }
}
