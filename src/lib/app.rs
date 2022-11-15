
use std::convert::Infallible;
use std::net::SocketAddr;

use super::database::table_schema::TableSchema;

use super::api_http_server::routing::{Route, split_uri_args};
use super::api_http_server::middleware::Middleware;
use super::database::interfaces::DatabaseInterface;

use hyper::{Body, Request, Response, StatusCode};


pub struct App {
    pub routes: Vec<Box<dyn Route + Send + Sync>>,
    pub middleware: Vec<Box<dyn Middleware + Send + Sync>>,
    pub database_interface: Box<dyn DatabaseInterface + Send + Sync>,
}

impl App {
    fn match_route(&self, uri: String) -> Option<&TableSchema> {
        for route in &self.routes {
            let route_match = route.matches_uri(uri.clone());
            if route_match {
                return Some(route.get_schema())
            }
        }
        None
    }

    pub async fn handle_http_request(&self, req: Request<Body>, addr: SocketAddr) -> Result<Response<Body>, Infallible> {
        if req.method() == hyper::Method::OPTIONS {
            let response = Response::builder()
                .header("Allow", "OPTIONS, GET, POST, DELETE, PATCH")
                .header("Accept", "application/json")
                .body(
                    Body::empty()
                );
            return Ok(response.unwrap())
        }

        let mut req = req;

        log::info!("Request ({}) at {}", addr, req.uri());

        let (base_uri, _) = split_uri_args(req.uri().to_string());

        let table_schema = self.match_route(base_uri);

        for middleware in &self.middleware {
            middleware.process_request(&mut req);
        }

        let response: Response<Body> = match table_schema {
            None => {
                let response = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(
                        Body::from("Route not found")
                    );
                
                if response.is_err() {
                    log::error!("Error creating 404 response");
                    return Ok(Response::new(Body::from("Error creating response")));
                };
                response.unwrap()
            },
            Some(table_schema) => {
                self.database_interface.process_api_request(&mut req, table_schema).await
            }
        };
        Ok(response)
    }
}
