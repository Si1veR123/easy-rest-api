use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::database::table_schema::TableSchema;
use super::database::interfaces::SQLType;

use toml::Value;

const DEFAULT_CONFIG_PATH: &str = "server_config.toml";

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

pub fn read_config(optional_path: Option<&str>) -> (HashMap<String, String>, HashMap<String, TableSchema>) {
    // return general config HashMap<String, String>
    // and Vec of (route string, table schema)
    let config_path = Path::new(
        match optional_path {
            Some(p) => p,
            None => DEFAULT_CONFIG_PATH
        }
    );

    let mut buffer = Vec::new();
    let _res = File::open(config_path).expect("Can't open config file").read_to_end(&mut buffer);

    let file_string = String::from_utf8(buffer).expect("Can't construct string from bytes in config file");
    let mut toml_parsed = file_string.parse::<Value>().expect("Can't pass toml config");

    let toml_main_table = toml_parsed.as_table_mut().expect("Toml config couldn't be parsed as table");
    let mut table_data = toml_main_table
        .remove("table")
        .expect("No tables found in config");

    let table_data = table_data
        .as_table_mut()
        .expect("Tables in config aren't of toml 'Table' type");

    let mut general_config = HashMap::new();
    for config in toml_main_table {
        let value = config.1.as_str();
        if value.is_none() {
            panic!("Encountered non-string value in toml config")
        }

        general_config.insert(config.0.clone(), value.unwrap().to_string());
    }

    let mut table_routes = HashMap::new();
    for table in table_data {
        let table_name = table.0;

        let table_attributes = table.1.as_table_mut().expect("Contents of table aren't of toml 'Table' type");

        let route = table_attributes.remove("route")
            .expect("Table has no 'route'");

        let route = route
            .as_str()
            .expect("Route value isn't 'String'");

        let mut table_schema_mapping = HashMap::new();
        
        for field in table_attributes {
            let sql_type_string = field.1.as_str().expect("Encountered non-string SQL type value").to_ascii_lowercase();
            let field_sql_type = match sql_type_string.as_str() {
                "null" => Some(SQLType::Null),
                "real" => Some(SQLType::Real),
                "integer" => Some(SQLType::Integer),
                "text" => Some(SQLType::Text),
                _ => None
            };
            if field_sql_type.is_none() {
                panic!("Invalid SQL type found in table field: {}", sql_type_string);
            }
            table_schema_mapping.insert(field.0.clone(), field_sql_type.unwrap());
        }

        table_routes.insert(route.to_string(), TableSchema {name: table_name.clone(), fields: table_schema_mapping});
    }

    (general_config, table_routes)
}
