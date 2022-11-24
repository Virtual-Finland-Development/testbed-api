use http::{HeaderMap, StatusCode, HeaderValue};
use reqwest::Response;
use serde_json::json;
use std::fs;

use crate::api::{
    routing_types::{APIRoutingError, APIRoutingResponse, ParsedRequest},
    utils::{get_cors_response_headers, get_default_headers, get_plain_headers}
};

pub async fn cors_preflight_response(
    _request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse::new(StatusCode::OK, "", get_cors_response_headers()))
}

pub async fn index(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse::new(StatusCode::TEMPORARY_REDIRECT, "Redirecting to /docs", {
        let mut headers = HeaderMap::new();
        headers.insert("Location", HeaderValue::from_static("/docs"));
        headers
    }))
}

pub async fn docs(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let body = fs::read_to_string("./openapi/index.html").expect("Unable to read index.html file");
    Ok(APIRoutingResponse::new(StatusCode::OK, body.as_ref(), {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("text/html"));
        headers
    }))
}

pub async fn openapi_spec(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let body = fs::read_to_string("./openapi/openapi.yml").expect("Unable to read openapi.yml file");
    Ok(APIRoutingResponse::new(StatusCode::OK, body.as_ref(), {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/octet-stream"));
        headers
    }))
}

pub async fn health_check(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse::new(StatusCode::OK, "OK", get_plain_headers()))
}

pub async fn not_found(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse {
        status_code: StatusCode::NOT_FOUND,
        body: json!({
            "message": "Not Found".to_string(),
        })
        .to_string(),
        headers: get_default_headers(),
    })
}

pub async fn get_external_service_bad_response(
    response: Response,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let status_code = response.status();
    let response_body = response.text().await?;
    return resolve_external_service_bad_response(status_code, response_body);
}

pub fn resolve_external_service_bad_response(
    status_code: StatusCode,
    response_body: String,
) -> Result<APIRoutingResponse, APIRoutingError> {

    // Ensure JSON response
    let response_json = serde_json::from_str::<serde_json::Value>(&response_body)
    .unwrap_or(json!({"content": response_body}));
    
    Ok(APIRoutingResponse {
        status_code: status_code,
        body: json!({
            "message": format!("External service responded with a status: {}", status_code).to_string(),
            "data": response_json,
        })
        .to_string(),
        headers: get_default_headers(),
    })
}