use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

const DEFAULT_CONFIG_PATH: &str = "server_config.ini";

fn parse_line(line: String) -> Result<(String, String), u8> {
    // Err(0) is invalid config line (could be empty)

    let mut split = line.split('=');

    let k = match split.nth(0) {
        Some(val) => val.to_string(),
        None => {return Err(0)}
    };

    let val_unclipped = match split.nth(0) {
        Some(val) => val.to_string(),
        None => {return Err(0)}
    };

    // TODO: check if \n or \r exists
    // strip \n and \r
    let v = val_unclipped[0..val_unclipped.len()-2].to_string();

    Ok((k, v))
}

pub fn read_config(optional_path: Option<&str>) -> HashMap<String, String> {
    let config_path = Path::new(
        match optional_path {
            Some(p) => p,
            None => DEFAULT_CONFIG_PATH
        }
    );

    let mut config_file = BufReader::new(File::open(config_path).expect("Can't open config file"));
    let mut config = HashMap::new();
    
    loop {
        let mut buffer = String::new();
        match config_file.read_line(&mut buffer) {
            Err(e) => panic!("{}", e),
            Ok(n) => {
                if n == 0 {
                    break
                }
            },
        };
        match parse_line(buffer) {
            Ok(val) => config.insert(val.0, val.1),
            Err(_) => {continue}
        };
    }

    config
}
