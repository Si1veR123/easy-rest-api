use std::collections::HashMap;

use super::interfaces::SQLType;

#[derive(Debug)]
pub struct SqlTableSchema {
    pub name: String,

    // col name: data type
    pub fields: HashMap<String, SQLType>
}

impl SqlTableSchema {
    pub fn field_exists(&self, field_name: &str) -> bool {
        // id field is always present
        self.fields.contains_key(field_name) || field_name == "id"
    }
}
