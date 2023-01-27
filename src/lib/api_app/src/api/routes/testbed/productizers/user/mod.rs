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
        ::var("USER_PROFILE_PRODUCTIZER_ENDPOINT")
        .expect("USER_PROFILE_PRODUCTIZER_ENDPOINT must be set");
    let request_input = json!({}); // Empty body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}

pub async fn fetch_user_status_info(
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env
        ::var("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT")
        .expect("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT must be set");
    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or(json!({})); // Pass through body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}

pub async fn update_user_status_info(
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env
        ::var("USER_STATUS_INFO_WRITE_PRODUCTIZER_ENDPOINT")
        .expect("USER_STATUS_INFO_WRITE_PRODUCTIZER_ENDPOINT must be set");
    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or(json!({})); // Pass through body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}