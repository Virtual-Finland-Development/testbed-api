use http::{HeaderMap, HeaderValue};
use std::env;

use app::{responses::APIRoutingError, router::ParsedRequest};
use utils::environment::get_stage;

pub mod figure;
pub mod job;
pub mod nsg;
pub mod person;
pub mod user;

/**
 * Parses the authorization headers from the input request
 */
fn parse_testbed_request_headers(request: ParsedRequest) -> Result<HeaderMap, APIRoutingError> {
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
fn build_data_product_uri(data_product: &str, data_source: &str) -> String {
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
            _ => {}
        }
    }

    // Remove trailing slash from base url
    if testbed_base_url.ends_with('/') {
        testbed_base_url.pop();
    }

    format!("{testbed_base_url}/{data_product}?source={data_source}:{testbed_environment}")
}
