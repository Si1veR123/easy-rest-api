use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use super::super::app::App;

use log::error;

async fn handle(
    context: Arc<App>,
    addr: SocketAddr,
    req: Request<Body>
) -> Result<Response<Body>, Infallible> {
    context.handle_http_request(req, addr)
}

pub async fn run_app_server(addr: SocketAddr, app: App) {
    let context = Arc::new(app);

    // A `MakeService` that produces a `Service` to handle each connection.
    let make_service = make_service_fn(move |conn: &AddrStream| {

        let context = context.clone();

        let addr = conn.remote_addr();

        let service = service_fn(move |req| {
            handle(context.clone(), addr, req)
        });

        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
