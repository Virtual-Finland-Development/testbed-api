use serde_json::{Value as JSONValue, json};

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

const TESTBED_BASE_URL: &str = "https://gateway.testbed.fi/";

pub async fn get_data_product(data_product: &str, request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let request_input = json!({});
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        TESTBED_BASE_URL.to_owned() + data_product,
        &request_input,
        request_headers,
    ).await?;
    Ok(response)
}
