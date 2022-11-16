use std::collections::HashMap;
use std::fmt::Display;

use hyper::body::to_bytes;
use hyper::{Request, Body, Method};

use sqlite3::{Cursor, Connection};
use sqlite3::Result as SqlResult;
use sqlite3::Value as SqlValue;
use sqlite3::Error as SqlError;

use super::super::api_http_server::routing::split_uri_args;
use super::table_schema::TableSchema;

use json::parse;

#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PATCH,
    INVALID
}

#[derive(Debug)]
pub struct QueryErr (
    pub String,  // description
    pub bool,  // server fault?
);

impl Display for QueryErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// Used to convert the incoming HTTP request to a SQL statement
#[async_trait::async_trait]
pub trait Query<'a, T, A> {
    async fn from_request(request: &mut Request<Body>, table: &TableSchema) -> Result<Self, QueryErr>
        where Self: Sized;
    fn execute_sql(&'a self, connection: T) -> A;
}

pub struct Sqlite3Query {
    pub method: HttpMethod,
    pub table_name: String,
    pub fields_data: HashMap<String, String>,
    pub filter: HashMap<String, String>,
}

#[async_trait::async_trait]
impl<'a> Query<'a, &'a Connection, SqlResult<Cursor<'a>>> for Sqlite3Query {
    
    async fn from_request(request: &mut Request<Body>, table: &TableSchema) -> Result<Self, QueryErr> {
        let method = match request.method().clone() {
            Method::GET => HttpMethod::GET,
            Method::PATCH => HttpMethod::PATCH,
            Method::DELETE => HttpMethod::DELETE,
            Method::POST => HttpMethod::POST,
            _ => HttpMethod::INVALID,
        };

        if method == HttpMethod::INVALID {
            return Err(QueryErr("Invalid Method".to_string(), false))
        }

        if method == HttpMethod::GET {
            // GET is constructed from uri args

            let (_, uri_args) = split_uri_args(request.uri().to_string());

            let mut uri_args_parsed: HashMap<String, String> = HashMap::new();
            for arg in uri_args.split('&') {
                let res = arg.split_once('=');

                if res.is_none() {
                    continue;
                }

                let (left, right) = res.unwrap();

                if table.field_exists(left) {
                    uri_args_parsed.insert(left.to_string(), right.to_string());
                }
            }

            return Ok(Self {
                method: HttpMethod::GET,
                table_name: table.name.clone(),
                fields_data: HashMap::new(),
                filter: uri_args_parsed,
            })
        }

        // TODO: possible vunerability in to_bytes
        let body_read_result = to_bytes(request.body_mut()).await;
        if body_read_result.is_err() {
            return Err(QueryErr("Error reading request body".to_string(), true))
        }
        let body = String::from_utf8(body_read_result.unwrap().into_iter().collect());
        if body.is_err() {
            return Err(QueryErr("Error creating string from request body bytes".to_string(), true))
        }

        let body = body.unwrap();
        let parsed = parse(
            &body
        );
        if parsed.is_err() {
            let error = parsed.err().unwrap();
            return Err(QueryErr(format!("Error parsing json ( {} ): {}", body, error), false))
        }
        
        let mut content = parsed.unwrap();
        let columns = content.remove("columns");
        if !columns.is_object() {
            return Err(QueryErr("Error getting 'columns' from json (not present or wrong type)".to_string(), false));
        }

        let mut data_hashmap = HashMap::new();
        for col in columns.entries() {
            let col_as_str = col.1.as_str();

            if col_as_str.is_none() {
                return Err(QueryErr("Columns json contains non-string".to_string(), false))
            }

            data_hashmap.insert(col.0.to_string(), col_as_str.unwrap().to_string());
        }

        let filters = content.remove("filters");
        if !filters.is_object() {
            return Err(QueryErr("Error getting 'filters' from json (not present or wrong type)".to_string(), false))
        }

        let mut filters_hashmap = HashMap::new();
        for filter in filters.entries() {
            let filter_val = filter.1.as_str();
            if filter_val.is_none() {
                return Err(QueryErr("Filters json contains non-string".to_string(), false))
            }

            filters_hashmap.insert(filter.0.to_string(), filter_val.unwrap().to_string());
        }

        Ok(Self {
            method,
            table_name: table.name.clone(),
            fields_data: data_hashmap,
            filter: filters_hashmap
        })
    }

    fn execute_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor<'a>> {
        match self.method {
            HttpMethod::GET => self.construct_get_sql(connection),
            HttpMethod::POST => self.construct_post_sql(connection),
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
        let mut select_builder = "SELECT *".to_string();

        select_builder.push_str(&format!(" FROM {}", self.table_name.clone()));

        if self.filter.len() > 0 {
            select_builder.push_str(" WHERE ");

            for filter in &self.filter {
                // fields MUST be checked to be valid for the table when constructing query object
                // or vulnerable to SQL injection
                select_builder.push_str( &format!("{}=? AND ", filter.0) );

                bindings.push(SqlValue::String(filter.1.clone()));
            }

            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
            select_builder.remove(select_builder.len()-1);
        }

        let statement = connection.prepare(select_builder);
        
        if statement.is_err() {
            let error = statement.err().unwrap();
            return Err(error)
        }

        let mut bound = statement.unwrap().cursor();
        let _res = bound.bind(bindings.as_slice());

        Ok(bound)
    }

    fn construct_post_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }
    
    fn construct_delete_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }

    fn construct_patch_sql(&'a self, connection: &'a Connection) -> SqlResult<Cursor> {
        Ok(connection.prepare("INVALID TEST STATEMENT").unwrap().cursor())
    }
}
