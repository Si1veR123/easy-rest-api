pub mod config_parser;
pub mod database;
pub mod api_http_server;
pub mod app;

use std::collections::HashMap;

use log::{warn};

pub fn enable_logging(config: &HashMap<String, String>) {
    let init = match config.get("loglevel") {
        Some(val) => {
            let env = env_logger::Env::new();
            let env = env.default_filter_or(val);
            env_logger::try_init_from_env(env)
        }
        None => {
            env_logger::try_init()
        }
    };
    match init {
        Ok(_) => (),
        Err(e) => warn!("{}", e)
    }
}
