use http::{HeaderMap, HeaderValue};

use crate::api::{responses::APIRoutingError, utils::ParsedRequest};

pub mod figure;
pub mod job;
pub mod person;
pub mod user;

/**
 * Parses the authorization headers from the input request
 */
fn parse_testbed_request_headers(request: ParsedRequest) -> Result<HeaderMap, APIRoutingError> {
    // Prep auth header forwarding
    let mut request_headers = HeaderMap::new();
    request_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
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
