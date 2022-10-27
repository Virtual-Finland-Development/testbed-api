use http::Response;
use lambda_http::Request;
use log;

use crate::api::utils::parse_router_request;

mod routes;
mod utils;

/**
 * The handler function for the lambda.
 */
pub async fn handler(
    request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    let parsed_request = parse_router_request(request);

    log::info!("{} {}", parsed_request.method, parsed_request.path);
    let router_response = routes::exec_router_request(parsed_request).await;
    log::debug!("Response: {:#?}", router_response);

    let mut api_response = Response::builder()
        .status(router_response.status_code)
        .body(router_response.body)
        .unwrap();

    // Populate headers
    let headers = api_response.headers_mut();
    for (key, value) in router_response.headers {
        headers.insert(key.unwrap(), value);
    }

    Ok(api_response)
}
