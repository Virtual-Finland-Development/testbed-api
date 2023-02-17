use std::collections::HashMap;
use utoipa::ToSchema;

use http::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;

use crate::api::routes::application::get_external_service_bad_response;

use openapi_router::{
    requests::ParsedRequest,
    responses::{APIResponse, APIRoutingError, APIRoutingResponse},
};
use utils::api::get_cors_response_headers;

pub mod productizers;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct ProxyRequestInput {
    method: String,
    url: String,
    body: String,
    headers: HashMap<String, String>,
}

#[utoipa::path(
    post,
    path = "/testbed/reverse-proxy",
    request_body(content = ProxyRequestInput, description = "Proxy request"),
    responses((status = 200, description = "Proxy response", content_type = "application/json"))
)]
pub async fn engage_reverse_proxy_request(request: ParsedRequest) -> APIResponse {
    let request_body_as_text = request.body.as_str();
    log::debug!("Input: {:#?}", request_body_as_text);
    let request_input: ProxyRequestInput =
        serde_json::from_str(request_body_as_text).expect("Failed to parse the request body");

    // Access control list check
    let access_denied = access_control_check(request_input.url.as_str());
    if access_denied {
        return Err(APIRoutingError::Unauthorized(
            "Unknown destination".to_string(),
        ));
    }

    // Transform headers
    let proxy_headers = HeaderMap::try_from(&request_input.headers)?;

    // Execute request
    let response = reqwest::Client::new()
        .post(request_input.url)
        .body(request_input.body)
        .headers(proxy_headers)
        .send()
        .await?;

    log::debug!("Response: {:#?}", response);

    let response_status = response.status();
    if response_status != 200 {
        return get_external_service_bad_response(response).await;
    }

    let response_output = response.json::<JSONValue>().await?;

    Ok(APIRoutingResponse::new(
        response_status,
        &serde_json::to_string(&response_output)?,
        get_cors_response_headers(),
    ))
}

/**
 * Access control check
 *
 * @param proxy_destination_url
 * @returns {boolean} - true if access is denied
 */
fn access_control_check(proxy_destination_url: &str) -> bool {
    // Access control list check
    let acl = ["https://consent.testbed.fi/", "https://gateway.testbed.fi/"];

    let mut acl_is_satisfied = false;
    for url in acl {
        if proxy_destination_url.starts_with(url) {
            acl_is_satisfied = true;
            break;
        }
    }

    !acl_is_satisfied
}
