use std::collections::HashMap;

use super::fields::{SQLType};

pub struct Table {
    name: String,

    // col name: data type
    fields: HashMap<String, SQLType>
}

impl Table {
    pub fn generate_create_sql(&self) -> String {
        let mut statement = format!("CREATE TABLE IF NOT EXISTS {} (", self.name);

        for field in &self.fields {
            statement.push_str(field.0);
            statement.push(' ');
            statement.push_str(&match *field.1 {
                SQLType::Null => "NULL",
                SQLType::Integer => "INTEGER",
                SQLType::Real => "REAL",
                SQLType::Text => "TEXT"
            });
            statement.push(',')
        }
        statement.remove(statement.len()-1);
        statement.push_str(");");
        statement
    }
}
