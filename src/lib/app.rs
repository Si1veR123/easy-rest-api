
use std::convert::Infallible;
use std::net::SocketAddr;

use super::api_http_server::routing::Route;
use super::middleware::Middleware;
use super::database::interfaces::DatabaseInterface;

use hyper::{Body, Request, Response, StatusCode};


pub struct App {
    pub routes: Vec<Box<dyn Route + Send + Sync>>,
    pub middleware: Vec<Box<dyn Middleware + Send + Sync>>,
    pub database_interface: Box<dyn DatabaseInterface + Send + Sync>,
}

impl App {
    fn match_route(&self, uri: String) -> Option<String> {
        for route in &self.routes {
            let route = route.matches_uri(uri.clone());
            if route.is_some() {
                return route
            }
        }
        None
    }

    pub async fn handle_http_request(&self, req: Request<Body>, addr: SocketAddr) -> Result<Response<Body>, Infallible> {
        log::info!("Request ({}) at {}", addr, req.uri());

        let table_name = self.match_route(req.uri().to_string());

        let response: Response<Body> = match table_name {
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
            Some(table_name) => {
                self.database_interface.process_api_request(req, &table_name).await
            }
        };
        Ok(response)
    }
}
