use http::{HeaderMap, HeaderValue};

use crate::api::{
    responses::APIRoutingError,
    utils::ParsedRequest,
};

pub mod figure;
pub mod job;
pub mod user;

/**
 * Parses the authorization headers fromn the input request
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
            .ok_or(APIRoutingError::UnprocessableEntity(
                "No authorization header".to_string(),
            ))?
            .clone(),
    );
    Ok(request_headers)
}
