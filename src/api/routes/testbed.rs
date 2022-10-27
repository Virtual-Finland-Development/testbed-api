use std::{collections::HashMap, str::FromStr};

use http::{header::HeaderName, HeaderMap, HeaderValue, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JSONValue};

use crate::api::{
    routes::application::get_external_service_bad_response,
    utils::{get_cors_response_headers, APIRoutingResponse, ParsedRequest},
};

#[derive(Deserialize, Serialize, Debug)]
struct ProxyRequestInput {
    method: String,
    url: String,
    data: HashMap<String, String>,
    headers: HashMap<String, String>,
}

pub async fn engage_reverse_proxy_request(request: ParsedRequest) -> APIRoutingResponse {
    let request_body_as_text = request.body.as_str();
    log::debug!("Input: {:#?}", request_body_as_text);
    let request_input: ProxyRequestInput = serde_json::from_str(request_body_as_text).unwrap();

    // Access control list check
    let access_denied = access_control_check(request_input.url.as_str());
    if access_denied {
        return APIRoutingResponse {
            status_code: StatusCode::UNAUTHORIZED,
            body: json!({
                "message": "Access Denied".to_string(),
            })
            .to_string(),
            headers: get_cors_response_headers(),
        };
    }

    // Transform headers
    let mut proxy_headers = HeaderMap::new();
    for (key, value) in request_input.headers {
        proxy_headers.insert(
            HeaderName::from_str(key.as_str()).unwrap(),
            HeaderValue::from_str(value.as_str()).unwrap(),
        );
    }

    let response = reqwest::Client::new()
        .post(request_input.url)
        .body(serde_json::to_string(&request_input.data).unwrap())
        .headers(proxy_headers)
        .send()
        .await
        .unwrap();

    log::debug!("Response: {:#?}", response);

    let response_status = response.status();
    if response_status != 200 {
        return get_external_service_bad_response(response_status);
    }

    let response_output = response.json::<JSONValue>().await.unwrap();
    return APIRoutingResponse {
        status_code: response_status,
        body: serde_json::to_string(&response_output).unwrap(),
        headers: get_cors_response_headers(),
    };
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
    for url in acl.iter() {
        if proxy_destination_url.starts_with(url) {
            acl_is_satisfied = true;
            break;
        }
    }

    return !acl_is_satisfied;
}
