use std::env;

use app::{requests::post_json_request, responses::APIResponse, router::ParsedRequest};
use utils::api::get_default_headers;

pub mod models;
use models::{RecommendationsRequest, RecommendationsResponse};

#[utoipa::path(
    post,
    path = "/jmf/recommendations",
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
pub async fn fetch_jmf_recommendations(request: ParsedRequest) -> APIResponse {
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
