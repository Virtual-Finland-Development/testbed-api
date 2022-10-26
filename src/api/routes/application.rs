use lambda_http::Request;
use serde_json::json;

use crate::api::APIRoutingResponse;

pub async fn index(_request: Request) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: 200,
        body: json!({
            "message": "Index".to_string(),
        }),
        headers: Default::default(),
    };
}

pub async fn not_found(_request: Request) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: 404,
        body: json!({
            "message": "Not Found".to_string(),
        }),
        headers: Default::default(),
    };
}
