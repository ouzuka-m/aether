use limine::request::ExecutableCmdlineRequest;
use spin::lazylock::LazyLock;

use crate::log::level::Level;

static CMDLINE_REQUEST: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let mut config = Config::default();
    let Some(cmdline) = CMDLINE_REQUEST.response() else {
        return config;
    };

    let cmdline_str = cmdline.cmdline();

    for arg in cmdline_str.split_whitespace() {
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
