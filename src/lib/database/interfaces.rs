use std::{collections::HashMap};

use std::fs;
use super::fields::DataType;
use super::fields::DataType::*;
use sqlite3::{open, Connection};

use log::{info, error};

type Config = HashMap<String, String>;

pub trait DatabaseInterface {
    fn connect(config: &Config) -> Self
        where Self: Sized;
    fn table_from_types(&self, table_name: String, types: Vec<(String, DataType)>);
    fn delete_db(config: &Config)
        where Self: Sized;
    fn execute_raw_query(&self, query: String);

    
}

pub struct SQLite3Interface {
    connection: Connection,
}

impl DatabaseInterface for SQLite3Interface {
    fn connect(config: &Config) -> Self {
        let db_path = config.get("database_path").expect("Can't find 'database_path' config");
        let connection = open(db_path).expect(&format!("Can't open sqllite3 database at {}", db_path));
        
        log::info!("Connected to database at {}", db_path);
        Self {
            connection
        }
    }
    fn table_from_types(&self, table_name: String, types: Vec<(String, DataType)>) {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);

        for (col_name, data_type) in types {
            sql.push_str(&col_name);
            let dtype = match data_type {
                Null => " NULL",
                Integer(_) => " INTEGER",
                Real(_) => " REAL",
                Text(_) => " TEXT",
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
            Ok(_) => info!("Deleted sqllite3 database"),
            Err(e) => error!("Error when deleting sqllite3 database: {}", e)
        }
    }
    fn execute_raw_query(&self, _query: String) {
        
    }
}

// may be negative side effects
// must be implemented for 'object safety' over threads
unsafe impl Send for SQLite3Interface {}
unsafe impl Sync for SQLite3Interface {}
