use http::StatusCode;
use serde_json::json;

use crate::api::{APIRoutingResponse, ParsedRequest};

pub async fn find_job_postings(_request: ParsedRequest<'_>) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: StatusCode::OK,
        body: json!({
            "message": "POP".to_string(),
        }),
        headers: Default::default(),
    };
}
