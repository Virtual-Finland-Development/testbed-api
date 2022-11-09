use http::header::HeaderMap;
use log;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::api::errors::APIRoutingError;
use crate::api::routes::application::get_external_service_bad_response;
use crate::api::routes::testbed::testbed_request_utils::parse_testbed_request_headers;
use crate::api::utils::{get_cors_response_headers, APIRoutingResponse, ParsedRequest};

/**
 * Population query parameters
 */

#[derive(Deserialize, Serialize, Debug)]
struct PopulationQuery {
    city: String,
    year: String, // Note: front apps send strings, not numbers
}

/**
 * Population response
 */
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct PopulationResponse {
    description: String,
    sourceName: String,
    population: i128,
    updatedAt: String,
}

/**
 * Get population figure
 */
pub async fn get_population(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let request_input: PopulationQuery = serde_json::from_str(request.body.as_str()).unwrap();
    let request_headers = parse_testbed_request_headers(request)?;
    return fetch_population(request_input, request_headers).await;
}

async fn fetch_population(
    request_input: PopulationQuery,
    request_headers: HeaderMap,
) -> Result<APIRoutingResponse, APIRoutingError> {
    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    let response = reqwest::Client::new()
        .post("https://gateway.testbed.fi/test/lsipii/Figure/Population?source=virtual_finland")
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

    let response_output = response.json::<PopulationResponse>().await.unwrap();
    return Ok(APIRoutingResponse {
        status_code: response_status,
        body: serde_json::to_string(&response_output).unwrap(),
        headers: get_cors_response_headers(),
    });
}
