use http::header::HeaderMap;
use http::{Response, StatusCode};
use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::{Request, RequestExt};
use serde_json::Value as JSONValue;

mod routes;

pub struct APIRoutingResponse {
    pub status_code: StatusCode, // http status code, e.g. 200, 404, 500
    pub body: JSONValue,         // Only serde_json::Value - values are supported
    pub headers: HeaderMap,
}

pub struct ParsedRequest<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub query: QueryMap,
    pub headers: HeaderMap,
    pub body: String,
}

/**
 * The handler function for the lambda.
 */
pub async fn handler(
    request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    let router_response = exec_router_request(request).await;
    let api_response = Response::builder()
        .status(router_response.status_code)
        .body(router_response.body.to_string())
        .unwrap();

    Ok(api_response)
}

/**
 * This is the router for the API.
 */
async fn exec_router_request(request: Request) -> APIRoutingResponse {
    let parsedRequest = parse_router_request(request);

    match (parsedRequest.method, parsedRequest.path) {
        ("GET", "/") => {
            return routes::application::index(parsedRequest).await;
        }
        ("GET", "/getPopulation ") => {
            return routes::figure::get_population(parsedRequest).await;
        }
        ("GET", "/findJobPostings ") => {
            return routes::job::find_job_postings(parsedRequest).await;
        }
        _ => {
            return routes::application::not_found(parsedRequest).await;
        }
    }
}

/**
 * Convert the lambda_http::Request to a ParsedRequest.
 */
fn parse_router_request(request: Request) -> ParsedRequest<'static> {
    let path = request.uri().path();
    let method = request.method().as_str();
    let query = request.query_string_parameters();
    let headers = request.headers().clone();
    let body = serde_json::from_slice(request.body()).unwrap(); // request.body().to_string();

    return ParsedRequest {
        path,
        method,
        query,
        headers,
        body,
    };
}
