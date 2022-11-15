use std::collections::HashMap;
use std::path::Path;
use std::fs;

use super::table_schema::TableSchema;
use super::response::{Sqlite3ResponseBuilder, ResponseBuilder};
use super::query::{Sqlite3Query, Query, QueryErr};

use sqlite3::{open, Connection};
use hyper::{Body, Request, Response};

#[derive(Clone, Debug)]
pub enum SQLType {
    Null,
    Integer,
    Real,
    Text,
}


type Config = HashMap<String, String>;

#[async_trait::async_trait]
pub trait DatabaseInterface {
    // (connection, existing?)
    fn connect(config: &Config) -> (Self, bool)
        where Self: Sized;
    fn create_tables_from_schemas(&self, schemas: Vec<&TableSchema>);
    fn table_from_types(&self, table_name: String, types: &HashMap<String, SQLType>);
    fn delete_db(config: &Config)
        where Self: Sized;
    async fn process_api_request(&self, request: &mut Request<Body>, table: &TableSchema) -> Response<Body>;
}

pub struct SQLite3Interface {
    connection: Connection,
}

#[async_trait::async_trait]
impl DatabaseInterface for SQLite3Interface {
    fn connect(config: &Config) -> (Self, bool) {
        let db_path = config.get("database_path").expect("Can't find 'database_path' config");

        let existing = Path::new(db_path).exists();

        let connection = open(db_path).expect(&format!("Can't open sqlite3 database at: {}", db_path));
        
        log::info!("Connected to database at {}", db_path);
        (
            Self {
                connection
            }
            , existing
        )
    }

    fn create_tables_from_schemas(&self, schemas: Vec<&TableSchema>) {
        for schema in schemas {
            self.table_from_types(schema.name.clone(), &schema.fields)
        }
    }

    fn table_from_types(&self, table_name: String, types: &HashMap<String, SQLType>) {
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

        log::info!("Creating table with SQL: {}", sql);

        self.connection.execute(sql).expect("Can't create table");
    }
    
    fn delete_db(config: &Config) {
        let db_path = config.get("database_path").expect("Can't find 'database_path' config");
        let result = fs::remove_file(db_path);

        match result {
            Ok(_) => log::info!("Deleted sqlite3 database"),
            Err(e) => log::error!("Error when deleting sqlite3 database: {}", e)
        }
    }

    async fn process_api_request(&self, request: &mut Request<Body>, table: &TableSchema) -> Response<Body> {
        // GENERATE QUERY
        let query = Sqlite3Query::from_request(request, table).await;

        if query.is_err() {
            let error = query.err().unwrap();

            if error.1 {
                log::warn!("{}", error.0);
                return Response::builder()
                    .status(500)
                    .body(Body::from("Server Error Encountered"))
                    .unwrap();
            } else {
                log::debug!("{}", error.0);
                return Response::builder()
                    .status(400)
                    .body(Body::from("Client Error Encountered"))
                    .unwrap();
            }
        }

        // EXECUTE QUERY
        let query = query.unwrap();
        let cursor = query.execute_sql(&self.connection);

        if cursor.is_err() {
            let error = cursor.err().unwrap();
            log::warn!("{}", error);
            
            return Response::builder()
                    .status(500)
                    .body(Body::from("Server Error Encountered"))
                    .unwrap();
        }

        // CREATE RESPONSE FROM DATA
        let mut all_data = Vec::new();
        let mut c = cursor.unwrap();
        while let Ok(Some(d)) = c.next() {
            all_data.push(d.to_vec())
        }

        let response_json_text = Sqlite3ResponseBuilder::from_row_data(all_data);

        Response::new(Body::from(response_json_text))
    }
}

// may be negative side effects
// must be implemented for 'object safety' over threads
unsafe impl Send for SQLite3Interface {}
unsafe impl Sync for SQLite3Interface {}
