use sqlite3::Value;

use json::JsonValue;

pub trait ResponseBuilder<T> {
    fn from_row_data(row_data: Vec<Vec<T>>) -> String;
}

pub struct Sqlite3ResponseBuilder;
impl ResponseBuilder<Value> for Sqlite3ResponseBuilder {
    fn from_row_data(row_data: Vec<Vec<Value>>) -> String {
        let mut root = JsonValue::Array(vec![]);
        
        for row in row_data {
            let mut row_root = JsonValue::Array(vec![]);

            for val in row {
                let json_value = match val {
                    Value::Float(f) => JsonValue::Number(json::number::Number::from(f)),
                    Value::Integer(i) => JsonValue::Number(json::number::Number::from(i)),
                    Value::String(s) => JsonValue::String(s),
                    Value::Null => JsonValue::Null,
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
