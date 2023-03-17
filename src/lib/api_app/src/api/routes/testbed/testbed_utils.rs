use http::{HeaderMap, HeaderValue};
use serde_json::{json, Value as JSONValue};
use std::env;

use app::{
    requests::post_json_request,
    responses::{APIResponse, APIRoutingError},
    router::ParsedRequest,
};
use utils::environment::get_stage;

/**
 *
 */
pub async fn post_data_product(
    data_product: &str,
    data_source: &str,
    request: ParsedRequest,
) -> APIResponse {
    let request_input: JSONValue =
        serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({}));
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        build_data_product_uri(data_product, data_source),
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}

/**
 * Parses the authorization headers from the input request
 */
pub fn parse_testbed_request_headers(
    request: ParsedRequest,
) -> Result<HeaderMap, APIRoutingError> {
    // Prep auth header forwarding
    let mut request_headers = HeaderMap::new();
    request_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    request_headers.insert(
        "authorization",
        request
            .headers
            .get("authorization")
            .ok_or_else(|| {
                APIRoutingError::UnprocessableEntity("No authorization header".to_string())
            })?
            .clone(),
    );

    if request.headers.contains_key("x-consent-token") {
        request_headers.insert(
            "x-consent-token",
            request
                .headers
                .get("x-consent-token")
                .ok_or_else(|| {
                    APIRoutingError::UnprocessableEntity(
                        "No x-consent-token header".to_string(),
                    )
                })?
                .clone(),
        );
    }
    Ok(request_headers)
}

/**
 * Builds the URI for the testbed data product
 */
pub fn build_data_product_uri(data_product: &str, data_source: &str) -> String {
    let mut testbed_base_url =
        env::var("TESTBED_BASE_URL").expect("TESTBED_BASE_URL must be set");
    let testbed_environment =
        env::var("TESTBED_ENVIRONMENT").expect("TESTBED_ENVIRONMENT must be set");

    if get_stage() == "local" {
        // @TODO: needs a local testbed data product gw simulation
        match data_product {
            "test/lassipatanen/User/Profile" => {
                testbed_base_url = env::var("USER_PROFILE_PRODUCTIZER_ENDPOINT")
                    .expect("USER_PROFILE_PRODUCTIZER_ENDPOINT must be set");
            }
            "test/lsipii/User/StatusInfo" => {
                testbed_base_url = env::var("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT")
                    .expect("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT must be set");
            }
            "test/lsipii/User/StatusInfo/Write" => {
                testbed_base_url = env::var("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT")
                    .expect("USER_STATUS_INFO_PRODUCTIZER_ENDPOINT must be set");
            }
            "draft/NSG/Agent/LegalEntity/NonListedCompany/Establishment/Write" => {
                testbed_base_url = env::var("PRH_MOCK_PRODUCTIZER_ENDPOINT")
                    .expect("PRH_MOCK_PRODUCTIZER_ENDPOINT must be set");
            }
            "draft/NSG/Agent/LegalEntity/NonListedCompany/BeneficialOwners" => {
                testbed_base_url = env::var("PRH_MOCK_PRODUCTIZER_ENDPOINT")
                    .expect("PRH_MOCK_PRODUCTIZER_ENDPOINT must be set");
            }
            "draft/NSG/Agent/LegalEntity/NonListedCompany/SignatoryRights" => {
                testbed_base_url = env::var("PRH_MOCK_PRODUCTIZER_ENDPOINT")
                    .expect("PRH_MOCK_PRODUCTIZER_ENDPOINT must be set");
            }
            _ => {}
        }
    }

    // Remove trailing slash from base url
    if testbed_base_url.ends_with('/') {
        testbed_base_url.pop();
    }

    format!("{testbed_base_url}/{data_product}?source={data_source}:{testbed_environment}")
}

/**
 * Access control check
 *
 * @param proxy_destination_url
 * @returns {boolean} - true if access is denied
 */
pub fn access_control_check(proxy_destination_url: &str) -> bool {
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
