use serde_json::{json, Value as JSONValue};

use crate::api::{
    requests::post_json_request,
    responses::{APIRoutingError, APIRoutingResponse},
    utils::ParsedRequest,
};

use super::parse_testbed_request_headers;

const TESTBED_BASE_URL: &str = "https://gateway.testbed.fi/";

pub async fn get_basic_information(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/BasicInformation";
    let request_input = json!({});
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        TESTBED_BASE_URL.to_owned() + data_product,
        &request_input,
        request_headers,
    ).await?;
    Ok(response)
}

pub async fn post_basic_information(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/BasicInformation/Write";
    let request_input: JSONValue = serde_json::from_str(request.body.as_str()).unwrap_or(json!({}));
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        TESTBED_BASE_URL.to_owned() + data_product,
        &request_input,
        request_headers,
    ).await?;
    Ok(response)
}

pub async fn try_get_basic_information(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/BasicInformation";
    let result = super::get_data_product(data_product, request).await?;
    Ok(result)
}
