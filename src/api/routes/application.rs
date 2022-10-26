use http::StatusCode;
use serde_json::json;

use crate::api::{APIRoutingResponse, ParsedRequest};

pub async fn index(_request: ParsedRequest<'_>) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: StatusCode::OK,
        body: json!({
            "message": "Index".to_string(),
        }),
        headers: Default::default(),
    };
}

pub async fn not_found(_request: ParsedRequest<'_>) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: StatusCode::NOT_FOUND,
        body: json!({
            "message": "Not Found".to_string(),
        }),
        headers: Default::default(),
    };
}
