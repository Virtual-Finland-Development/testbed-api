use http::HeaderMap;

use crate::api::ParsedRequest;

/**
 * Parses the authorization headers fromn the input request
 */
pub fn parse_testbed_request_headers(request: ParsedRequest) -> HeaderMap {
    // Prep auth header forwarding
    let mut request_headers = HeaderMap::new();
    request_headers.insert("Content-Type", "application/json".parse().unwrap());
    request_headers.insert(
        "authorization",
        request.headers.get("authorization").unwrap().clone(),
    );
    request_headers.insert(
        "x-authorization-provider",
        request
            .headers
            .get("x-authorization-provider")
            .unwrap()
            .clone(),
    );

    return request_headers;
}
