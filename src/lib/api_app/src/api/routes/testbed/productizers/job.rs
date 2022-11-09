use http::header::HeaderMap;
use log;
use reqwest;
use serde_json::Value as JSONValue;

use crate::api::routes::application::get_external_service_bad_response;
use crate::api::routes::testbed::testbed_request_utils::parse_testbed_request_headers;
use crate::api::utils::{get_cors_response_headers, APIRoutingResponse, ParsedRequest};

/**
 * Get job postings
 */
pub async fn find_job_postings(request: ParsedRequest) -> APIRoutingResponse {
    let request_input = serde_json::from_str(request.body.as_str()).unwrap();
    let request_headers = parse_testbed_request_headers(request);
    return fetch_job_postings(request_input, request_headers).await;
}

async fn fetch_job_postings(
    request_input: JSONValue,
    request_headers: HeaderMap,
) -> APIRoutingResponse {
    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    let response = reqwest::Client::new()
        .post("https://gateway.testbed.fi/test/lassipatanen/Job/JobPosting?source=tyomarkkinatori")
        .json(&request_input)
        .headers(request_headers)
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
