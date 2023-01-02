use std::env;

use crate::api::{
    requests::post_json_request,
    responses::{APIRoutingError, APIRoutingResponse},
    utils::{get_default_headers, ParsedRequest},
};

mod models;
use models::{RecommendationsRequest, RecommendationsResponse};

pub async fn fetch_jmf_recommendations(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env::var("JMF_SKILL_RECOMMENDATIONS_ENDPOINT")
        .expect("JMF_SKILL_RECOMMENDATIONS_ENDPOINT must be set");
    let request_input: RecommendationsRequest = serde_json::from_str(request.body.as_str())?;
    let response = post_json_request::<RecommendationsRequest, RecommendationsResponse>(
        endpoint_url,
        &request_input,
        get_default_headers(),
    )
    .await?;
    Ok(response)
}
