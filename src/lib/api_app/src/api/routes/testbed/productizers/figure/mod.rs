use std::env;

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse },
    requests::post_json_request,
    utils::ParsedRequest,
};
use super::parse_testbed_request_headers;

mod figure_models;
use figure_models::{ PopulationQuery, PopulationResponse };

/**
 * Get population figure
 */
pub async fn get_population(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env
        ::var("POPULATION_FIGURE_PRODUCTIZER_ENDPOINT")
        .expect("POPULATION_FIGURE_PRODUCTIZER_ENDPOINT must be set");
    let request_input: PopulationQuery = serde_json::from_str(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<PopulationQuery, PopulationResponse>(
        endpoint_url.to_string(),
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}