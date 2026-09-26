//! Kernel runtime configuration.
//!
//! Parses the kernel command line (supplied by the bootloader) to
//! extract configuration parameters such as the log verbosity level.

use spin::lazylock::LazyLock;

use crate::{boot::info::CMDLINE, log::level::Level};

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let mut config = Config::default();
    let cmdline = *CMDLINE;
    if cmdline.is_empty() {
        return config;
    }

    for arg in cmdline.split_whitespace() {
        if let Some(value) = arg.strip_prefix("log_level=") {
            match value {
                "debug" => config.set_log_level(Level::Debug),
                "info" => config.set_log_level(Level::Info),
                "warn" => config.set_log_level(Level::Warn),
                "error" => config.set_log_level(Level::Error),
                _ => {}
            }
        }
    }

    config
});

#[derive(Debug)]
pub struct Config {
    pub log_level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: Level::Info,
        }
    }
}

impl Config {
    pub fn set_log_level(&mut self, log_level: Level) {
        self.log_level = log_level;
    }
}
