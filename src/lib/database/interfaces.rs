use std::collections::HashMap;

use std::fs;
use super::query::{Sqlite3Query, Query};
use sqlite3::{open, Connection};

#[derive(Clone)]
pub enum SQLType {
    Null,
    Integer,
    Real,
    Text,
}


use hyper::{Body, Request, Response};

type Config = HashMap<String, String>;

#[async_trait::async_trait]
pub trait DatabaseInterface {
    fn connect(config: &Config) -> Self
        where Self: Sized;
    fn table_from_types(&self, table_name: String, types: Vec<(String, SQLType)>);
    fn delete_db(config: &Config)
        where Self: Sized;
    fn execute_raw_query(&self, query: String);
    async fn process_api_request(&self, request: Request<Body>, table: &str) -> Response<Body>;
}

pub struct SQLite3Interface {
    connection: Connection,
}

#[async_trait::async_trait]
impl DatabaseInterface for SQLite3Interface {
    fn connect(config: &Config) -> Self {
        let db_path = config.get("database_path").expect("Can't find 'database_path' config");
        let connection = open(db_path).expect(&format!("Can't open sqllite3 database at {}", db_path));
        
        log::info!("Connected to database at {}", db_path);
        Self {
            connection
        }
    }
    fn table_from_types(&self, table_name: String, types: Vec<(String, SQLType)>) {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);

        for (col_name, data_type) in types {
            sql.push_str(&col_name);
            let dtype = match data_type {
                SQLType::Null => " NULL",
                SQLType::Integer => " INTEGER",
                SQLType::Real => " REAL",
                SQLType::Text => " TEXT",
            };
            sql.push_str(dtype);
            sql.push_str(", ")
        }
        // remove last ", "
        sql.remove(sql.len()-1);
        sql.remove(sql.len()-1);
        sql.push_str(");");

        log::info!("Creating table with SQL {}", sql);

        self.connection.execute(sql).expect("Can't create table");
    }
    fn delete_db(config: &Config) {
        let db_path = config.get("database_path").expect("Can't find 'database_path' config");
        let result = fs::remove_file(db_path);

        match result {
            Ok(_) => log::info!("Deleted sqllite3 database"),
            Err(e) => log::error!("Error when deleting sqllite3 database: {}", e)
        }
    }
    fn execute_raw_query(&self, _query: String) {
        
    }
    async fn process_api_request(&self, request: Request<Body>, table: &str) -> Response<Body> {
        let query = Sqlite3Query::from_request(&request, table).await;

        if query.is_err() {
            log::error!("Failed to construct SQL query from request");
            // return error response
        }

        let cursor = query.unwrap().execute_sql(&self.connection);
        if cursor.is_err() {
            log::error!("Failed to execute SQL query");
            // return error response
        }


    }
}

// may be negative side effects
// must be implemented for 'object safety' over threads
unsafe impl Send for SQLite3Interface {}
unsafe impl Sync for SQLite3Interface {}
