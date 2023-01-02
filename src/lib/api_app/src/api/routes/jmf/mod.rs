use std::env;

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse },
    requests::post_json_request,
    utils::ParsedRequest,
};

mod models;
use http::HeaderMap;
use models::{ RecommendationsRequest, RecommendationsResponse };

pub async fn fetch_jmf_recommendations(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = env
        ::var("JMF_SKILL_RECOMMENDATIONS_ENDPOINT")
        .expect("JMF_SKILL_RECOMMENDATIONS_ENDPOINT must be set");
    let request_input: RecommendationsRequest = serde_json::from_str(request.body.as_str())?;
    let request_headers = HeaderMap::new();
    let response = post_json_request::<RecommendationsRequest, RecommendationsResponse>(
        endpoint_url,
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}