use serde_json::{Value as JSONValue, json};

use crate::api::{
    requests::post_json_request,
    responses::{APIRoutingError, APIRoutingResponse},
    utils::ParsedRequest,
};

use super::parse_testbed_request_headers;

const TESTBED_BASE_URL: &str = "https://gateway.testbed.fi/";

pub async fn get_job_applicant_profile(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/JobApplicantProfile";
    let request_input = json!({});
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        TESTBED_BASE_URL.to_owned() + data_product,
        &request_input,
        request_headers,
    ).await?;
    Ok(response)
}

pub async fn post_job_applicant_profile(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/JobApplicantProfile/Write";
    let request_input = serde_json::from_str(request.body.as_str()).unwrap_or(json!({}));
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        TESTBED_BASE_URL.to_owned() + data_product,
        &request_input,
        request_headers,
    ).await?;
    Ok(response)
}

