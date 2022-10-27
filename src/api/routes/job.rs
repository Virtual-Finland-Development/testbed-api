use http::StatusCode;
use serde_json::json;

use crate::api::{get_cors_response_headers, APIRoutingResponse, ParsedRequest};

pub async fn find_job_postings(_request: ParsedRequest) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: StatusCode::OK,
        body: json!({
            "message": "POP".to_string(),
        })
        .to_string(),
        headers: get_cors_response_headers(),
    };
}
