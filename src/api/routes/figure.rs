use http::header::HeaderMap;
use log;
use reqwest;

use crate::api::{get_cors_response_headers, APIRoutingResponse, ParsedRequest};
use serde::{Deserialize, Serialize};

/**
 * Population query parameters
 */

#[derive(Deserialize, Serialize, Debug)]
struct PopulationQuery {
    city: String,
    year: i8,
}

/**
 * Population response
 */
#[derive(Deserialize, Serialize, Debug)]
struct PopulationResponse {
    description: String,
    source_name: String,
    population: i128,
    updated_at: String,
}

/**
 * Get population figure
 */
pub async fn get_population(request: ParsedRequest) -> APIRoutingResponse {
    let request_input: PopulationQuery = serde_json::from_str(request.body.as_str()).unwrap();
    let request_headers = request.headers;

    return fetch_population(request_input, request_headers).await;
}

async fn fetch_population(
    request_input: PopulationQuery,
    request_headers: HeaderMap,
) -> APIRoutingResponse {
    let response = reqwest::Client::new()
        .post("https://gateway.testbed.fi/test/lsipii/Figure/Population?source=virtual_finland")
        .json(&request_input)
        .headers(request_headers)
        .send()
        .await
        .unwrap();
    println!();
    log::debug!("{:#?}", response);

    let response_status = response.status();
    let response_output = response.json::<PopulationResponse>().await.unwrap();

    return APIRoutingResponse {
        status_code: response_status,
        body: serde_json::to_string(&response_output).unwrap(),
        headers: get_cors_response_headers(),
    };
}
