use serde_json::{ Value as JSONValue, json };

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse },
    requests::post_json_request,
    utils::ParsedRequest,
};
use super::{ parse_testbed_request_headers, build_data_product_uri };

pub async fn fetch_user_profile(
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = build_data_product_uri(
        "test/lassipatanen/User/Profile",
        "access_to_finland"
    );
    let request_input = json!({}); // Empty body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url,
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}

pub async fn fetch_user_status_info(
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = build_data_product_uri("test/lsipii/User/StatusInfo", "virtual_finland");
    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({})); // Pass through body
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
    let endpoint_url = build_data_product_uri(
        "test/lsipii/User/StatusInfo/Write",
        "virtual_finland"
    );

    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({})); // Pass through body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}