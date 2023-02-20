use http::{
    header::{HeaderMap, HeaderName},
    HeaderValue,
};

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
