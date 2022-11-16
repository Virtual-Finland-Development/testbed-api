use http::header::HeaderMap;
use log;
use reqwest;
use serde_json::Value as JSONValue;

use crate::api:: {
    routing_types::{APIRoutingError, APIRoutingResponse, ParsedRequest},
    routes::application::get_external_service_bad_response,
    utils::get_default_headers
};
use super::parse_testbed_request_headers;

/**
 * Get job postings
 */
pub async fn find_job_postings(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = "https://gateway.testbed.fi/test/lassipatanen/Job/JobPosting?source=tyomarkkinatori";
    let request_input = serde_json::from_str(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    return fetch_job_postings(endpoint_url, request_input, request_headers).await;
}

async fn fetch_job_postings(
    endpoint_url: &str,
    request_input: JSONValue,
    request_headers: HeaderMap,
) -> Result<APIRoutingResponse, APIRoutingError> {
    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    let response = reqwest::Client::new()
        .post(endpoint_url)
        .json(&request_input)
        .headers(request_headers)
        .send()
        .await?;

    log::debug!("Response: {:#?}", response);

    let response_status = response.status();
    if response_status != 200 {
        return get_external_service_bad_response(response_status);
    }

    let response_output = response.json::<JSONValue>().await?;

    Ok(APIRoutingResponse::new(response_status, &serde_json::to_string(&response_output)?, get_default_headers()))
}
