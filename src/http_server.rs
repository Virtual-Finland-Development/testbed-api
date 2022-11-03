use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lambda_http::Body as LambdaBody;
use std::convert::Infallible;
use std::net::SocketAddr;

use crate::api;

async fn handle(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let lambda_body = LambdaBody::from(request.data());

    let response = api::handler(_req).await;
    let httpResponse = Response::new(Body::from(response.body));
    Ok(httpResponse)
}

pub async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
