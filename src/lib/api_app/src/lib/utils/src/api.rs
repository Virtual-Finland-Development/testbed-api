use std::collections::HashMap;

use http::{
    header::{HeaderMap, HeaderName},
    HeaderValue,
};
use lambda_http::aws_lambda_events::query_map::QueryMap;

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
        HeaderValue::from_static("GET, POST, DELETE, OPTIONS"),
    );

    headers.insert(
        HeaderName::from_static("access-control-allow-headers"),
        HeaderValue::from_static(
            "content-type, authorization, x-authorization-provider, x-authorization-context, x-consent-token"
        )
    );

    headers
}

pub fn get_default_headers() -> HeaderMap {
    let mut cors_headers = get_cors_response_headers();

    cors_headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );

    cors_headers
}

pub fn get_plain_headers() -> HeaderMap {
    let mut cors_headers = get_cors_response_headers();

    cors_headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/plain"),
    );

    cors_headers
}

pub fn parse_query_param(query: QueryMap, query_param_name: &str) -> Result<String, String> {
    let query_param = query.first(query_param_name);
    if query_param.is_none() {
        return Err(format!("Missing {} parameter", query_param_name));
    }

    Ok(query_param.unwrap().to_string())
}

pub fn parse_path_param(
    path_params: HashMap<String, String>,
    path_param_name: &str,
) -> Result<String, String> {
    let path_param = path_params.get(path_param_name);
    if path_param.is_none() {
        return Err(format!("Missing {} path parameter", path_param_name));
    }

    Ok(path_param.unwrap().to_string())
}
