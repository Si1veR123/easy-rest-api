pub mod config_parser;
pub mod database;
pub mod api_http_server;
pub mod app;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;

use std::collections::HashMap;

pub fn enable_logging(config: &HashMap<String, String>) {
    // panic if called more than once

    let _init = match config.get("loglevel") {
        Some(val) => {

            Builder::new()
                .format(|buf, record| {
                    writeln!(buf,
                        "{} [{}] - {}",
                        Local::now().format("%Y-%m-%dT%H:%M:%S"),
                        record.level(),
                        record.args()
                    )
                })
                .parse_filters(val)
                .init()
        }
        None => {
            let _ = env_logger::try_init();
        }
    };
}
