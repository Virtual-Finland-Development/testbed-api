use http::StatusCode;
use serde_json::json;

use crate::api::utils::{get_cors_response_headers, APIRoutingResponse, ParsedRequest};

pub async fn cors_preflight_response(_request: ParsedRequest) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: StatusCode::OK,
        body: "".to_string(),
        headers: get_cors_response_headers(),
    };
}

pub async fn index(_request: ParsedRequest) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: StatusCode::OK,
        body: json!({
            "message": "Index".to_string(),
        })
        .to_string(),
        headers: Default::default(),
    };
}

pub async fn not_found(_request: ParsedRequest) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: StatusCode::NOT_FOUND,
        body: json!({
            "message": "Not Found".to_string(),
        })
        .to_string(),
        headers: get_cors_response_headers(),
    };
}

pub fn get_external_service_bad_response(status_code: StatusCode) -> APIRoutingResponse {
    return APIRoutingResponse {
        status_code: status_code,
        body: json!({
            "message": format!("External service responded with a status: {}", status_code).to_string(),
        })
        .to_string(),
        headers: get_cors_response_headers(),
    };
}
