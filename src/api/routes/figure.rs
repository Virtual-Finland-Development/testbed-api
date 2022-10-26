use http::header::HeaderMap;
use reqwest;
use serde_json::Value;

use crate::api::{APIRoutingResponse, ParsedRequest};
use serde::{Deserialize, Serialize};

/**
 * Population query parameters
 */

#[derive(Serialize)]
struct PopulationQuery {
    city: String,
    year: i8,
}

/**
 * Population response
 */
#[derive(Deserialize)]
struct PopulationResponse {
    description: String,
    source_name: String,
    population: i128,
    updated_at: String,
}

pub async fn get_population(request: ParsedRequest<'_>) -> APIRoutingResponse {
    let requestInput: PopulationQuery =
        serde_json::from_str(request.body.as_str()).unwrap_or(PopulationQuery {
            city: "New York".to_string(),
            year: 2019,
        });

    let requestHeaders = request.headers;

    return fetch_population(requestInput, requestHeaders).await;
}

async fn fetch_population(
    requestInput: PopulationQuery,
    requestHeaders: HeaderMap,
) -> APIRoutingResponse {
    let response = reqwest::Client::new()
        .post("https://gateway.testbed.fi/test/lsipii/Figure/Population?source=virtual_finland")
        .json(&requestInput)
        .headers(requestHeaders)
        .send()
        .await?;
    println!("{:#?}", response);

    let responseStatus = response.status();
    let responseOutput = response.json::<PopulationResponse>().await;

    return APIRoutingResponse {
        status_code: responseStatus,
        body: responseOutput,
        headers: Default::default(),
    };
}
