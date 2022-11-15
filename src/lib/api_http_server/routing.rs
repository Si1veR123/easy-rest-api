use super::super::database::table_schema::TableSchema;


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
    fn matches_uri(&self, uri: String) -> Option<String>;
}

#[derive(Debug)]
pub struct BasicRoute {
    pub route: String,
    pub table_schema: TableSchema,
}

impl BasicRoute {
    pub fn new(route: String, table_schema: TableSchema) -> Self {
        Self {
            route,
            table_schema
        }
    }
}

impl Route for BasicRoute {
    fn matches_uri(&self, uri: String) -> Option<String> {
        match uri == self.route {
            false => None,
            true => Some(self.table_schema.name.clone())
        }
    }
}
