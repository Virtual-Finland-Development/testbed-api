use std::env;

use crate::api::{
    requests::post_json_request,
    responses::{APIRoutingError, APIRoutingResponse},
    utils::{get_default_headers, ParsedRequest},
};

pub mod models;
use models::{RecommendationsRequest, RecommendationsResponse};

#[utoipa::path(
    post,
    path = "/testbed/productizers/user-profile",
    request_body(
        content = RecommendationsRequest,
        description = "Job Market Finland recommended skills and occupations"
    ),
    responses((
        status = 200,
        body = RecommendationsResponse,
        description = "The recommendations response",
    ))
)]
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
