use http::header::{HeaderMap, HeaderName};
use http::HeaderValue;
use http::StatusCode;
use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::{Body, Request, RequestExt};
use serde_json::json;

use crate::api::errors::APIRoutingError;

#[derive(Debug)]
pub struct APIRoutingResponse {
    pub status_code: StatusCode, // http status code, e.g. 200, 404, 500
    pub body: String,
    pub headers: HeaderMap,
}

impl APIRoutingResponse {
    pub fn new(status_code: StatusCode, body: &str, headers: HeaderMap) -> Self {
        Self {
            status_code,
            body: body.to_string(),
            headers,
        }
    }

    pub fn from_routing_error(error: APIRoutingError) -> Self {
        Self::new(
            error.get_status_code(),
            json!({
                "message": error.to_string(),
            })
            .to_string()
            .as_ref(),
            get_cors_response_headers(),
        )
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
    let mut cors_headers = get_cors_response_headers();

    cors_headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );

    return cors_headers;
}

pub fn get_plain_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/plain"),
    );

    return headers;
}
