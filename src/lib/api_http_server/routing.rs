use super::super::database::table_schema::SqlTableSchema;


/// Example
/// ```
/// let routes = routes!(
///    ("/people", PeopleTableSchema),
///    ("/jobs", JobsTableSchema)
///);
/// ```
#[macro_export]
macro_rules! routes {
    ($( ($route:expr, $table:expr) ),*) => {{
        let mut vec = Vec::new();
        $( vec.push( Box::new(BasicRoute::new($route.to_string(), $table)) as Box<dyn Route + Send + Sync>); )*
        vec
    }};
}

pub trait Route {
    // returns matching table name for route
    fn matches_uri(&self, uri: String) -> bool;
    fn get_schema(&self) -> &SqlTableSchema;
}

#[derive(Debug)]
pub struct BasicRoute {
    pub route: String,
    pub table_schema: SqlTableSchema,
}

impl BasicRoute {
    pub fn new(route: String, table_schema: SqlTableSchema) -> Self {
        Self {
            route,
            table_schema
        }
    }
}

impl Route for BasicRoute {
    fn matches_uri(&self, uri: String) -> bool {
        uri == self.route
    }
    fn get_schema(&self) -> &SqlTableSchema {
        &self.table_schema
    }
}

pub fn split_uri_args(uri: String) -> (String, String) {
    // split at last ?
    let base_uri;
    let uri_args;

    let whole_uri = uri.to_string();
    let path = whole_uri.chars().rev().position(|x| x == '?');

    match path {
        None => {
            base_uri = whole_uri;
            uri_args = String::new();
        },
        Some(v) => {
            let (base, args) = whole_uri.split_at(whole_uri.len()-v);
            let stripped = base.strip_suffix('?');
            base_uri = stripped.unwrap_or(base).to_string();
            uri_args = args.to_string();
        }
    }
    (base_uri, uri_args)
}
