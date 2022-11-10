#[macro_export]
macro_rules! routes {
    ($( ($route:expr, $table:expr) ),*) => {{
        let mut vec = Vec::new();
        $( vec.push( Box::new(BasicRoute::new($route.to_string(), $table.to_string())) as Box<dyn Route + Send + Sync>); )*
        vec
    }};
}
pub use routes;

pub trait Route {
    // returns matching table name for route
    fn matches_uri(&self, uri: String) -> Option<String>;
}

pub struct BasicRoute {
    route: String,
    table_name: String,
}

impl BasicRoute {
    pub fn new(route: String, table_name: String) -> Self {
        Self {
            route,
            table_name
        }
    }
}

impl Route for BasicRoute {
    fn matches_uri(&self, uri: String) -> Option<String> {
        match uri == self.route {
            false => None,
            true => Some(self.table_name.clone())
        }
    }
}
