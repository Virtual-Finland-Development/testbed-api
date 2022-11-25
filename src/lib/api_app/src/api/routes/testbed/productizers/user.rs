use serde_json::{Value as JSONValue};

use crate::api:: {
    response_types::{APIRoutingError, APIRoutingResponse, ParsedRequest},
    requests::post_json_request
};
use super::parse_testbed_request_headers;


pub async fn fetch_user_profile(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = "https://gateway.testbed.fi/test/lassipatanen/User/Profile?source=access_to_finland";
    let request_input = serde_json::from_str(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    return post_json_request::<JSONValue, JSONValue>(endpoint_url, request_input, request_headers).await;
}
