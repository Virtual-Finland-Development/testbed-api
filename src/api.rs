use http::Response;
use lambda_http::Request;
use log;

mod routes;
mod testbed_requests;
mod utils;

/**
 * The handler function for the lambda.
 */
pub async fn handler(
    request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    let router_response = exec_router_request(request).await;
    let mut api_response = Response::builder()
        .status(router_response.status_code)
        .body(router_response.body)
        .unwrap();

    {
        let headers = api_response.headers_mut();
        for (key, value) in router_response.headers {
            headers.insert(key.unwrap(), value);
        }
    }

    Ok(api_response)
}

/**
 * This is the router for the API.
 */
async fn exec_router_request(request: Request) -> utils::APIRoutingResponse {
    let parsed_request = utils::parse_router_request(request);

    let method = parsed_request.method.as_str();
    let path = parsed_request.path.as_str();

    log::info!("{} {}", method, path);

    if method == "OPTIONS" {
        return routes::application::cors_preflight_response(parsed_request).await;
    }

    match (method, path) {
        ("GET", "/") => {
            return routes::application::index(parsed_request).await;
        }
        ("POST", "/getPopulation") => {
            return routes::figure::get_population(parsed_request).await;
        }
        ("POST", "/findJobPostings") => {
            return routes::job::find_job_postings(parsed_request).await;
        }
        _ => {
            return routes::application::not_found(parsed_request).await;
        }
    }
}
