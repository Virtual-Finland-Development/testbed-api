use http::header::HeaderMap;
use log;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::api:: {
    response_types::{APIRoutingError, APIRoutingResponse, ParsedRequest},
    routes::application::get_external_service_bad_response,
    utils::get_default_headers
};
use super::parse_testbed_request_headers;


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
    let endpoint_url = "https://gateway.testbed.fi/test/lsipii/Figure/Population?source=virtual_finland";
    let request_input: PopulationQuery = serde_json::from_str(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    return fetch_population(endpoint_url, request_input, request_headers).await;
}

async fn fetch_population(
    endpoint_url: &str,
    request_input: PopulationQuery,
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
        return get_external_service_bad_response(response).await;
    }

    let response_output = response.json::<PopulationResponse>().await?;
    Ok(APIRoutingResponse::new(response_status, &serde_json::to_string(&response_output)?, get_default_headers()))
}
