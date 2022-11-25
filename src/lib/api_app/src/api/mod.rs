use http::Response;
use lambda_http::Request;
use log;

mod routes;
mod response_types;
mod requests;

pub mod utils;
pub mod text_utils;

/**
 * The handler function for the lambda.
 */
pub async fn handler(
    request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    let parsed_request = utils::parse_router_request(request);

    log::info!("{} {}", parsed_request.method, parsed_request.path);
    let router_response = routes::exec_router_request(parsed_request).await;
    log::debug!("Response: {:#?}", router_response);

    let mut api_response = Response::builder()
        .status(router_response.status_code)
        .body(router_response.body)
        .expect("Failed to build handler response");

    // Populate headers
    let headers = api_response.headers_mut();
    for (key, value) in router_response.headers {
        headers.insert(key.expect("Bad header key in API-response"), value);
    }

    Ok(api_response)
}
