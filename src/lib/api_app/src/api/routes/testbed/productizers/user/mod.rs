use std::env;

use serde_json::{ Value as JSONValue, json };

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse },
    requests::post_json_request,
    utils::ParsedRequest,
};
use super::parse_testbed_request_headers;

pub async fn fetch_user_profile(
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env
        ::var("USERS_PRODUCTIZER_ENDPOINT")
        .expect("USERS_PRODUCTIZER_ENDPOINT must be set");
    let request_input = json!({}); // Empty body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}