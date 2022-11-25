use std::{collections::HashMap};

use http::{HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json::{Value as JSONValue};

use crate::api::{
    response_types::{APIRoutingError, APIRoutingResponse, ParsedRequest},
    routes::application::get_external_service_bad_response,
    utils::{get_default_headers},
};

pub mod productizers;

#[derive(Deserialize, Serialize, Debug)]
struct ProxyRequestInput {
    method: String,
    url: String,
    data: HashMap<String, String>,
    headers: HashMap<String, String>,
}

pub async fn engage_reverse_proxy_request(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let request_body_as_text = request.body.as_str();
    log::debug!("Input: {:#?}", request_body_as_text);
    let request_input: ProxyRequestInput = serde_json::from_str(request_body_as_text).unwrap();

    // Access control list check
    let access_denied = access_control_check(request_input.url.as_str());
    if access_denied {
        return Err(APIRoutingError::Unauthorized("Unknown destination".to_string()));
    }

    // Transform headers
    let proxy_headers = HeaderMap::try_from(&request_input.headers)?;
    
    // Execute request
    let response = reqwest::Client::new()
        .post(request_input.url)
        .body(serde_json::to_string(&request_input.data).unwrap())
        .headers(proxy_headers)
        .send()
        .await?;

    log::debug!("Response: {:#?}", response);

    let response_status = response.status();
    if response_status != 200 {
        return get_external_service_bad_response(response).await;
    }

    let response_output = response.json::<JSONValue>().await?;

    Ok(APIRoutingResponse::new(response_status, &serde_json::to_string(&response_output)?, get_default_headers()))
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

    return !acl_is_satisfied;
}
