use http::{HeaderMap, StatusCode};
use serde_json::json;
use std::fs;

use crate::api::errors::APIRoutingError;
use crate::api::utils::{get_cors_response_headers, get_default_headers, APIRoutingResponse, ParsedRequest};

pub async fn cors_preflight_response(
    _request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse {
        status_code: StatusCode::OK,
        body: "".to_string(),
        headers: get_cors_response_headers(),
    })
}

pub async fn index(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse {
        status_code: StatusCode::TEMPORARY_REDIRECT,
        body: "Redirecting to /docs".to_string(),
        headers: {
            let mut headers = HeaderMap::new();
            headers.insert("Location", "/docs".parse().unwrap());
            headers
        },
    })
}

pub async fn docs(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let body = fs::read_to_string("./openapi/index.html").unwrap();

    Ok(APIRoutingResponse {
        status_code: StatusCode::OK,
        body: body.to_string(),
        headers: {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html".parse().unwrap());
            headers
        },
    })
}

pub async fn openapi_spec(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let body = fs::read_to_string("./openapi/openapi.yml").unwrap();

    Ok(APIRoutingResponse {
        status_code: StatusCode::OK,
        body: body.to_string(),
        headers: {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "application/octet-stream".parse().unwrap());
            headers
        },
    })
}

pub async fn health_check(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse {
        status_code: StatusCode::OK,
        body: "OK".to_string(),
        headers: get_default_headers(),
    })
}

pub async fn not_found(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse {
        status_code: StatusCode::NOT_FOUND,
        body: json!({
            "message": "Not Found".to_string(),
        })
        .to_string(),
        headers: get_cors_response_headers(),
    })
}

pub fn get_external_service_bad_response(
    status_code: StatusCode,
) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse {
        status_code: status_code,
        body: json!({
            "message": format!("External service responded with a status: {}", status_code).to_string(),
        })
        .to_string(),
        headers: get_cors_response_headers(),
    })
}
