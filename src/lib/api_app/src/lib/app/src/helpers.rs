use http::{HeaderMap, HeaderValue};

use crate::{responses::APIRoutingError, router::ParsedRequest};
/**
 * Parses the authorization headers from the input request
 */
pub fn parse_request_headers(request: ParsedRequest) -> Result<HeaderMap, APIRoutingError> {
    // Prep auth header forwarding
    let mut request_headers = HeaderMap::new();
    request_headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    if request.headers.contains_key("authorization") {
        request_headers.insert(
            "authorization",
            request
                .headers
                .get("authorization")
                .ok_or_else(|| {
                    APIRoutingError::UnprocessableEntity("No authorization header".to_string())
                })?
                .clone(),
        );
    }

    if request.headers.contains_key("x-consent-token") {
        request_headers.insert(
            "x-consent-token",
            request
                .headers
                .get("x-consent-token")
                .ok_or_else(|| {
                    APIRoutingError::UnprocessableEntity(
                        "No x-consent-token header".to_string(),
                    )
                })?
                .clone(),
        );
    }
    Ok(request_headers)
}
