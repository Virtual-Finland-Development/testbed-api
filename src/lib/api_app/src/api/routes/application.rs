use http::{ HeaderMap, StatusCode, HeaderValue, Method };
use reqwest::Response;
use serde_json::{ json, Value as JsonValue };
use std::{ fs, env };

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse, resolve_external_service_bad_response },
    requests::engage_many_plain_requests,
    utils::{ ParsedRequest, get_cors_response_headers, get_default_headers, get_plain_headers },
};

pub async fn cors_preflight_response(
    _request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse::new(StatusCode::OK, "", get_cors_response_headers()))
}

#[utoipa::path(get, path = "/", responses((status = 303, description = "Redirect to /docs")))]
pub async fn index(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(
        APIRoutingResponse::new(StatusCode::TEMPORARY_REDIRECT, "Redirecting to /docs", {
            let mut headers = HeaderMap::new();
            headers.insert("Location", HeaderValue::from_static("/docs"));
            headers
        })
    )
}

#[utoipa::path(
    get,
    path = "/docs",
    responses((status = 200, description = "API documentation", body = String))
)]
pub async fn docs(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let body = fs::read_to_string("./openapi/index.html").expect("Unable to read index.html file");
    Ok(
        APIRoutingResponse::new(StatusCode::OK, body.as_ref(), {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", HeaderValue::from_static("text/html"));
            headers
        })
    )
}

pub async fn openapi_spec(json_spec: String) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(
        APIRoutingResponse::new(StatusCode::OK, json_spec.as_ref(), {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", HeaderValue::from_static("application/json"));
            headers
        })
    )
}

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Health check", body = String))
)]
pub async fn health_check(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse::new(StatusCode::OK, "OK", get_plain_headers()))
}

pub async fn not_found(_request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    Ok(APIRoutingResponse {
        status_code: StatusCode::NOT_FOUND,
        body: json!({
            "message": "Not Found".to_string(),
        }).to_string(),
        headers: get_default_headers(),
    })
}

pub async fn get_external_service_bad_response(
    response: Response
) -> Result<APIRoutingResponse, APIRoutingError> {
    let status_code = response.status();
    let response_body = response.text().await?;
    return resolve_external_service_bad_response(status_code, response_body);
}

#[utoipa::path(
    get,
    path = "/wake-up",
    responses((status = 200, description = "Wake signal handler", body = String))
)]
pub async fn wake_up_external_services(
    _request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoints = vec![
        format!("{}/health", env::var("AUTHENTICATION_GW_ENDPOINT_ORIGIN").unwrap_or_default()),
        format!("{}", env::var("USERS_API_ENDPOINT_ORIGIN").unwrap_or_default()),
        format!("{}/wake-up", env::var("TMT_PRODUCTIZER_ENDPOINT_ORIGIN").unwrap_or_default()),
        format!(
            "{}/wake-up",
            env::var("JOBS_IN_FINLAND_PRODUCTIZER_ENDPOINT_ORIGIN").unwrap_or_default()
        )
    ];
    let total_endpoints = endpoints.len();

    let wake_up_input = json!({
       "message": "Wake up!".to_string(),
    });

    let (response_status, good_responses, error_response_body) =
        engage_many_plain_requests::<JsonValue>(
            endpoints,
            Method::GET,
            &wake_up_input,
            get_default_headers(),
            true
        ).await?;

    if response_status == StatusCode::OK {
        return Ok(
            APIRoutingResponse::new(
                StatusCode::OK,
                &json!({"signals": {
                    "successful": good_responses.len(),
                    "total": total_endpoints,
                } }).to_string(),
                get_default_headers()
            )
        );
    }
    resolve_external_service_bad_response(response_status, error_response_body)
}