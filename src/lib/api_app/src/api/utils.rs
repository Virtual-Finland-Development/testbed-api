use http::header::{HeaderMap, HeaderName};
use http::HeaderValue;
use http::StatusCode;
use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::{Body, Request, RequestExt};

use crate::api::errors::APIRoutingError;

#[derive(Debug)]
pub struct APIRoutingResponse {
    pub status_code: StatusCode, // http status code, e.g. 200, 404, 500
    pub body: String,
    pub headers: HeaderMap,
}

impl APIRoutingResponse {
    pub fn from_routing_error(error: APIRoutingError) -> APIRoutingResponse {
        let status_code = error.get_status_code();
        let body = error.to_string();
        let headers = get_cors_response_headers();

        APIRoutingResponse {
            status_code,
            body,
            headers,
        }
    }
}

pub struct ParsedRequest {
    pub path: String,
    pub method: String,
    pub query: QueryMap,
    pub headers: HeaderMap,
    pub body: String,
}

/**
 * Convert the lambda_http::Request to a parsed_request.
 */
pub fn parse_router_request(request: Request) -> ParsedRequest {
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

pub fn get_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/plain"),
    );

    return headers;
}
