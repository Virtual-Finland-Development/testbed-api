use http::header::{HeaderMap, HeaderName};
use http::{HeaderValue, Response, StatusCode};
use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::{Body, Request, RequestExt};
use log;

mod routes;
mod testbed_requests;

pub struct APIRoutingResponse {
    pub status_code: StatusCode, // http status code, e.g. 200, 404, 500
    pub body: String,
    pub headers: HeaderMap,
}

pub struct ParsedRequest {
    pub path: String,
    pub method: String,
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
async fn exec_router_request(request: Request) -> APIRoutingResponse {
    let parsed_request = parse_router_request(request);

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

/**
 * Convert the lambda_http::Request to a parsed_request.
 */
fn parse_router_request(request: Request) -> ParsedRequest {
    let path = request.uri().path().clone().to_string();
    let method = request.method().as_str().to_string();
    let query = request.query_string_parameters().clone();
    let headers = request.headers().clone();

    // Body parsing is left to the route handlers, where the models are defined
    let body: String = match request.body() {
        Body::Text(body) => body.clone(),
        //Body::Binary(body) => serde_json::from_slice(body),
        _ => "".to_string(),
    };

    return ParsedRequest {
        path,
        method,
        query,
        headers,
        body,
    };
}

/**
 * Cors preflight response headers.
 */
pub fn get_cors_response_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        HeaderName::from_static("access-control-allow-origin"),
        HeaderValue::from_static("*"),
    );

    headers.insert(
        HeaderName::from_static("access-control-allow-methods"),
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );

    headers.insert(
        HeaderName::from_static("access-control-allow-headers"),
        HeaderValue::from_static(
            "content-type, authorization, x-authorization-provider, x-authorization-context",
        ),
    );

    return headers;
}
