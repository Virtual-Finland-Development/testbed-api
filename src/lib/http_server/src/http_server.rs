use hyper::service::{make_service_fn, service_fn};
use hyper::{
    Body as HyperBody, Request as HyperRequest, Response as HyperResponse, Server as HyperServer,
};
use lambda_http::{Body as LambdaBody, Request as LambdaRequest};
use std::convert::Infallible;
use std::net::SocketAddr;

use api_app::api;

async fn handle(request: HyperRequest<HyperBody>) -> Result<HyperResponse<HyperBody>, Infallible> {
    // Transform Hyper request into Lambda request
    let (parts, body) = request.into_parts();
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    let lambda_body = LambdaBody::from(body_text);
    let mut lambda_request = LambdaRequest::new(lambda_body);
    *lambda_request.method_mut() = parts.method;
    *lambda_request.uri_mut() = parts.uri;
    *lambda_request.version_mut() = parts.version;
    *lambda_request.headers_mut() = parts.headers;
    *lambda_request.extensions_mut() = parts.extensions;

    // Exec request handler
    let response = api::handler(lambda_request).await?;

    // Transform Lambda response into Hyper response
    let mut hyper_response = HyperResponse::builder()
        .status(response.status())
        .body(HyperBody::from(response.body().to_string()))
        .unwrap();

    // Set response headers
    let response_headers = hyper_response.headers_mut();
    for (key, value) in response.headers() {
        response_headers.insert(key, value.clone());
    }

    Ok(hyper_response)
}

pub async fn run() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = HyperServer::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
