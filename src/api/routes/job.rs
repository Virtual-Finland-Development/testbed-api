use lambda_http::Request;
use serde_json::json;

use crate::api::APIRoutingResponse;

pub async fn find_job_postings(_request: Request) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: 404,
        body: json!({
            "message": "POP".to_string(),
        }),
        headers: Default::default(),
    };
}
