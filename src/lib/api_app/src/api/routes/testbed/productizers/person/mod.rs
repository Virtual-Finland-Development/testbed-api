use std::env;

use serde_json::{json, Value as JSONValue};

use crate::{
    api::{
        requests::post_json_request,
        responses::{APIRoutingError, APIRoutingResponse},
        utils::ParsedRequest,
    }
};

use super::parse_testbed_request_headers;

pub mod basic_information;
pub mod job_applicant_profile;

fn build_data_product_uri(data_product: &str, data_source: &str) -> String {
    let testbed_base_url = env::var("TESTBED_BASE_URL").expect("TESTBED_BASE_URL must be set");
    let testbed_environment = env::var("TESTBED_ENVIRONMENT").expect("TESTBED_ENVIRONMENT must be set");
    let formatted_path = format!("{testbed_base_url}/{data_product}?source={data_source}:{testbed_environment}");
    formatted_path
}

pub async fn get_data_product(data_product: &str, data_source: &str, request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let request_input = json!({});
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        build_data_product_uri(data_product, data_source),
        &request_input,
        request_headers,
    ).await?;
    Ok(response)
}

pub async fn write_data_product(data_product: &str, data_source: &str, request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or(json!({}));
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        build_data_product_uri(data_product, data_source),
        &request_input,
        request_headers,
    ).await?;
    Ok(response)
}
