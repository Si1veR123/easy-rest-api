use sqlite3::Value as SqlValue;

use json::JsonValue;

pub trait ResponseBuilder<T> {
    fn from_row_data(row_data: Vec<Vec<T>>) -> String;
}

pub struct Sqlite3ResponseBuilder;
impl ResponseBuilder<SqlValue> for Sqlite3ResponseBuilder {
    fn from_row_data(row_data: Vec<Vec<SqlValue>>) -> String {
        let mut root = JsonValue::Array(vec![]);
        
        for row in row_data {
            let mut row_root = JsonValue::Array(vec![]);

            for val in row {
                let json_value = match val {
                    SqlValue::Float(f) => JsonValue::Number(json::number::Number::from(f)),
                    SqlValue::Integer(i) => JsonValue::Number(json::number::Number::from(i)),
                    SqlValue::String(s) => JsonValue::String(s),
                    SqlValue::Null => JsonValue::Null,
                    t => {
                        log::debug!("Invalid SQLite3 type: {:?}", t);
                        continue
                    }
                };
                let _ = row_root.push(json_value);
            }

            let _ = root.push(row_root);
        }

        root.to_string()
    }
}
