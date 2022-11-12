use std::collections::HashMap;
use sqlite3::Cursor;

pub enum HttpAction {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH
}

// Used to convert the incoming HTTP request to a SQL statement
pub trait Query<'a, T, A> {
    fn execute_sql(&'a self, connection: T) -> A;
}

pub struct Sqlite3Query {
    pub action: HttpAction,
    pub table_name: String,
    pub data: HashMap<String, String>,
    pub filter: HashMap<String, String>,
}

impl<'a> Query<'a, &'a sqlite3::Connection, sqlite3::Result<Cursor<'a>>> for Sqlite3Query {
    fn execute_sql(&'a self, connection: &'a sqlite3::Connection) -> sqlite3::Result<Cursor<'a>> {
        match self.action {
            HttpAction::GET => self.construct_get_sql(connection),
            HttpAction::POST => self.construct_post_sql(connection),
            HttpAction::PUT => self.construct_put_sql(connection),
            HttpAction::DELETE => self.construct_delete_sql(connection),
            HttpAction::PATCH => self.construct_patch_sql(connection),
        }
    }
}

impl<'a> Sqlite3Query {
    fn construct_get_sql(&'a self, connection: &'a sqlite3::Connection) -> sqlite3::Result<Cursor> {
        let mut bindings: Vec<sqlite3::Value> = Vec::new();
        let mut select_builder = "SELECT ".to_string();

        // in select, only key of hashmap has values, as only need column names
        for d in &self.data {
            select_builder.push_str("?, ");
            bindings.push(sqlite3::Value::String(d.0.clone()));
        }
        select_builder.remove(select_builder.len()-1);
        select_builder.remove(select_builder.len()-1);

        select_builder.push_str(" FROM ?");
        bindings.push(sqlite3::Value::String(self.table_name.clone()));

        if self.filter.len() > 0 {
            select_builder.push_str(" WHERE ");

            for filter in &self.filter {
                select_builder.push_str("?=? AND ");
                bindings.push(sqlite3::Value::String(filter.0.clone()));
                bindings.push(sqlite3::Value::String(filter.1.clone()));
            }

            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
        }

        println!("Executing {}", select_builder);

        let statement = connection.prepare(select_builder);
        println!("{:?}", bindings);
        if statement.is_err() {
            let error = statement.err().unwrap();
            log::error!("Error constructing GET: {}", error);
            return Err(error)
        }

        let mut bound = statement.unwrap().cursor();
        bound.bind(bindings.as_slice());

        Ok(bound)
    }

    fn construct_post_sql(&'a self, connection: &'a sqlite3::Connection) -> sqlite3::Result<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }

    fn construct_put_sql(&'a self, connection: &'a sqlite3::Connection) -> sqlite3::Result<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }
    
    fn construct_delete_sql(&'a self, connection: &'a sqlite3::Connection) -> sqlite3::Result<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }

    fn construct_patch_sql(&'a self, connection: &'a sqlite3::Connection) -> sqlite3::Result<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_sql_gen() {
        let mut data = HashMap::new();
        data.insert("data1".to_string(), "".to_string());
        data.insert("data2".to_string(), "".to_string());

        let mut filter = HashMap::new();
        //filter.insert("filter1".to_string(), "value1".to_string());
        //filter.insert("filter2".to_string(), "value2".to_string());
        //filter.insert("filter3".to_string(), "value3".to_string());

        let q = Sqlite3Query {
            action: HttpAction::GET,
            table_name: "test".to_string(),
            data,
            filter
        };

        let connection = sqlite3::open(":memory:").unwrap();

        connection.execute(
            "
            CREATE TABLE test (data1 TEXT, data2 TEXT);
            INSERT INTO test (data1, data2) VALUES ('retreived_val1', 'retrienved_val2')
            "
        ).unwrap();

        let mut r = q.execute_sql(&connection).unwrap();
        
        while let Ok(v) = r.next() {
            println!("{:?}", v.unwrap());
        }

        assert_eq!(true, "" == "");
    }
}
