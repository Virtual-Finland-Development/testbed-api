use lambda_http::aws_lambda_events::query_map::QueryMap;
use http::{
    HeaderValue,
    header::{HeaderMap, HeaderName}
};
use lambda_http::{Body, Request, RequestExt};

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
    let path = format!("/{}", strings::trim_left_slashes(request.uri().path().clone().to_string()));
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

pub mod strings {

    pub fn truncate_too_long_string(string: impl Into<String>, max_length: usize, postfix: &str) -> String {
        let text = string.into();
        if text.len() > max_length {
            return text[..max_length].to_string() + postfix;
        }
        return text;
    }
    
    pub fn cut_string_by_delimiter_keep_right(string: impl Into<String>, delimiter: &str) -> String {
        let text = string.into();
        let split = text.split(delimiter);
        let result = split.last().unwrap().to_string();
        return result;
    }

    pub fn trim_left_slashes(string: impl Into<String>) -> String {
        let text = string.into();
        let mut result = text.clone();
        while result.starts_with("/") {
            result = result[1..].to_string();
        }
        return result;
    }
}