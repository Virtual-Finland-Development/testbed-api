use serde_json::{json, Value as JSONValue};

use app::{
    router::ParsedRequest,
    requests::post_json_request,
    responses::APIResponse,
};

use super::{ parse_testbed_request_headers, build_data_product_uri };

#[utoipa::path(
    post,
    path = "/testbed/productizers/user-profile",
    request_body(
        content = Object, 
        description = "User profile query",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lassipatanen/User/Profile.json",
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "User profile response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lassipatanen/User/Profile.json",
        )))
    ))
)]
pub async fn fetch_user_profile(
    request: ParsedRequest,
) -> APIResponse {
    let endpoint_url = build_data_product_uri(
        "test/lassipatanen/User/Profile",
        "access_to_finland"
    );
    let request_input = json!({}); // Empty body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url,
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/testbed/productizers/fetch-user-status-info",
    request_body(
        content = Object,
        description = "Fetch user status info",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo.json",
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Status information response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo.json",
        )))
    ))
)]
pub async fn fetch_user_status_info(
    request: ParsedRequest,
) -> APIResponse {
    let endpoint_url = build_data_product_uri("test/lsipii/User/StatusInfo", "virtual_finland");
    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({})); // Pass through body
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
    request_body(
        content = Object,
        description = "Update user status info",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo/Write.json",
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Status information response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/test/lsipii/User/StatusInfo/Write.json",
        )))
    ))
)]
pub async fn update_user_status_info(
    request: ParsedRequest,
) -> APIResponse {
    let endpoint_url = build_data_product_uri(
        "test/lsipii/User/StatusInfo/Write",
        "virtual_finland"
    );

    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({})); // Pass through body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}
