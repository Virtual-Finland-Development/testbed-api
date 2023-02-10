use std::env;

use serde_json::{json, Value as JSONValue};

use super::parse_testbed_request_headers;
use crate::api::{
    requests::post_json_request,
    responses::{APIRoutingError, APIRoutingResponse},
    utils::ParsedRequest,
};

#[utoipa::path(
    post,
    path = "/testbed/productizers/user-profile",
    request_body(content = Object, description = "User profile query"),
    responses((status = 200, body = Object, description = "User profile response"))
)]
pub async fn fetch_user_profile(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env::var("USER_PROFILE_PRODUCTIZER_ENDPOINT")
        .expect("USER_PROFILE_PRODUCTIZER_ENDPOINT must be set");
    let request_input = json!({}); // Empty body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/testbed/productizers/fetch-user-status-info",
    request_body(content = Object, description = "Fetch user status info", examples(
        ( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo.json",
        ) )
    )),
    responses((status = 200, body = Object, description = "Status information response", examples(
        ( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo.json",
        ) )
    ),))
)]
pub async fn fetch_user_status_info(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env::var("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT")
        .expect("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT must be set");
    let request_input: JSONValue =
        serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({})); // Pass through body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/testbed/productizers/update-user-status-info",
    request_body(content = Object, description = "Update user status info", examples(
        ( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo/Write.json",
        ) )
    )),
    responses((
        status = 200,
        body = Object,
        description = "Status information response",
        examples(
            ( "Success" = (
                summary = "JSON example",
                value = json!("Loading.."),
                external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo/Write.json",
            ) )
        ),
    ))
)]
pub async fn update_user_status_info(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env::var("USER_STATUS_INFO_WRITE_PRODUCTIZER_ENDPOINT")
        .expect("USER_STATUS_INFO_WRITE_PRODUCTIZER_ENDPOINT must be set");
    let request_input: JSONValue =
        serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({})); // Pass through body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}
