use std::collections::HashMap;

use hyper::body::to_bytes;
use hyper::{Request, Body};

use sqlite3::{Cursor, Connection};
use sqlite3::Result as SqlResult;
use sqlite3::Value as SqlValue;
use sqlite3::Error as SqlError;

use json::parse;

#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    INVALID
}

// Used to convert the incoming HTTP request to a SQL statement
#[async_trait::async_trait]
pub trait Query<'a, T, A> {
    // TODO: restructure to return errors with strings and log in caller function
    async fn from_request(request: &mut Request<Body>, table: &str) -> Result<Self, ()>
        where Self: Sized;
    fn execute_sql(&'a self, connection: T) -> A;
}

pub struct Sqlite3Query {
    pub method: HttpMethod,
    pub table_name: String,
    pub data: HashMap<String, String>,
    pub filter: HashMap<String, String>,
}

#[async_trait::async_trait]
impl<'a> Query<'a, &'a Connection, SqlResult<Cursor<'a>>> for Sqlite3Query {
    async fn from_request(request: &mut Request<Body>, table: &str) -> Result<Self, ()> {
        let method = match request.method().clone() {
            hyper::Method::GET => HttpMethod::GET,
            hyper::Method::PATCH => HttpMethod::PATCH,
            hyper::Method::DELETE => HttpMethod::DELETE,
            hyper::Method::POST => HttpMethod::POST,
            hyper::Method::PUT => HttpMethod::PUT,
            _ => HttpMethod::INVALID,
        };

        if method == HttpMethod::INVALID {
            return Err(())
        }

        // TODO: possible vunerability in to_bytes
        let body_read_result = to_bytes(request.body_mut()).await;
        if body_read_result.is_err() {
            log::error!("Error reading request body");
            return Err(())
        }
        let body = String::from_utf8(body_read_result.unwrap().into_iter().collect());
        if body.is_err() {
            log::error!("Error creating string from request body bytes");
            return Err(())
        }

        let parsed = parse(
            &body.unwrap()
        );
        if parsed.is_err() {
            log::error!("Error parsing json body");
            return Err(())
        }
        
        let mut content = parsed.unwrap();
        let columns = content.remove("columns");
        if !columns.is_object() {
            log::error!("Error getting 'columns' from json (not present or wrong type)");
            return Err(());
        }

        let mut data_hashmap = HashMap::new();
        for col in columns.entries() {
            let col_as_str = col.1.as_str();

            if col_as_str.is_none() {
                log::error!("Columns json contains non-string");
                return Err(())
            }

            data_hashmap.insert(col.0.to_string(), col_as_str.unwrap().to_string());
        }

        let filters = content.remove("filters");
        if !filters.is_object() {
            log::error!("Error getting 'filters' from json (not present or wrong type)");
            return Err(())
        }

        let mut filters_hashmap = HashMap::new();
        for filter in filters.entries() {
            let filter_val = filter.1.as_str();
            if filter_val.is_none() {
                log::error!("Filters json contains non-string");
                return Err(())
            }

            filters_hashmap.insert(filter.0.to_string(), filter_val.unwrap().to_string());
        }

        Ok(Self {
            method,
            table_name: table.to_string(),
            data: data_hashmap,
            filter: filters_hashmap

        })
    }

    fn execute_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor<'a>> {
        match self.method {
            HttpMethod::GET => self.construct_get_sql(connection),
            HttpMethod::POST => self.construct_post_sql(connection),
            HttpMethod::PUT => self.construct_put_sql(connection),
            HttpMethod::DELETE => self.construct_delete_sql(connection),
            HttpMethod::PATCH => self.construct_patch_sql(connection),
            _  => SqlResult::Err(
                SqlError {code: None, message: Some("Invalid method".to_string())}
            )
        }
    }
}

impl<'a> Sqlite3Query {
    fn construct_get_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
        let mut bindings: Vec<SqlValue> = Vec::new();
        let mut select_builder = "SELECT ".to_string();

        // in select, only key of hashmap has values, as only need column names
        for d in &self.data {
            select_builder.push_str("?, ");
            bindings.push(SqlValue::String(d.0.clone()));
        }
        select_builder.remove(select_builder.len()-1);
        select_builder.remove(select_builder.len()-1);

        select_builder.push_str(" FROM ?");
        bindings.push(SqlValue::String(self.table_name.clone()));

        if self.filter.len() > 0 {
            select_builder.push_str(" WHERE ");

            for filter in &self.filter {
                select_builder.push_str("?=? AND ");
                bindings.push(SqlValue::String(filter.0.clone()));
                bindings.push(SqlValue::String(filter.1.clone()));
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

    fn construct_post_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }

    fn construct_put_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }
    
    fn construct_delete_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }

    fn construct_patch_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
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
            method: HttpMethod::GET,
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
