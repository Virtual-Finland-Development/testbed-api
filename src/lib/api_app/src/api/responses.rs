use std::str::FromStr;

use super::utils::get_default_headers;
use http::header::HeaderMap;
use http::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub struct APIRoutingResponse {
    pub status_code: StatusCode, // http status code, e.g. 200, 404, 500
    pub body: String,
    pub headers: HeaderMap,
}

impl APIRoutingResponse {
    pub fn new(status_code: StatusCode, body: &str, headers: HeaderMap) -> Self {
        Self {
            status_code,
            body: body.to_string(),
            headers,
        }
    }

    pub fn from_routing_error(error: APIRoutingError) -> Self {
        Self::new(
            error.get_status_code(),
            json!({
                "message": error.to_string(),
            }).to_string().as_ref(),
            get_default_headers()
        )
    }
}

/**
 * Exceptions
 */

#[derive(Debug)]
pub enum APIRoutingError {
    // 400
    BadRequest(String),
    // 401
    Unauthorized(String),
    // 403
    Forbidden(String),
    // 404
    NotFound(String),
    // 422
    UnprocessableEntity(String),
    // 500
    InternalServerError(String),
    // 502
    BadGateway(String),
    // 503
    ServiceUnavailable(String),
    // 504
    GatewayTimeout(String),
}

impl APIRoutingError {
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            APIRoutingError::BadRequest(_) => StatusCode::BAD_REQUEST,
            APIRoutingError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            APIRoutingError::Forbidden(_) => StatusCode::FORBIDDEN,
            APIRoutingError::NotFound(_) => StatusCode::NOT_FOUND,
            APIRoutingError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            APIRoutingError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIRoutingError::BadGateway(_) => StatusCode::BAD_GATEWAY,
            APIRoutingError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            APIRoutingError::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    #[allow(dead_code)] // Signify the meaning of default message
    pub fn from_status_code(status_code: StatusCode) -> Self {
        Self::from_status_code_and_message(status_code, "default")
    }

    pub fn from_status_code_and_message(
        status_code: StatusCode,
        message: impl Into<String>
    ) -> Self {
        let message_string = message.into();
        match status_code {
            StatusCode::BAD_REQUEST => APIRoutingError::BadRequest(message_string),
            StatusCode::UNAUTHORIZED => APIRoutingError::Unauthorized(message_string),
            StatusCode::FORBIDDEN => APIRoutingError::Forbidden(message_string),
            StatusCode::NOT_FOUND => APIRoutingError::NotFound(message_string),
            StatusCode::UNPROCESSABLE_ENTITY =>
                APIRoutingError::UnprocessableEntity(message_string),
            StatusCode::INTERNAL_SERVER_ERROR =>
                APIRoutingError::InternalServerError(message_string),
            StatusCode::BAD_GATEWAY => APIRoutingError::BadGateway(message_string),
            StatusCode::SERVICE_UNAVAILABLE => APIRoutingError::ServiceUnavailable(message_string),
            StatusCode::GATEWAY_TIMEOUT => APIRoutingError::GatewayTimeout(message_string),
            _ => APIRoutingError::InternalServerError(message_string),
        }
    }
}

impl std::error::Error for APIRoutingError {}

impl From<std::string::String> for APIRoutingError {
    fn from(e: std::string::String) -> Self {
        APIRoutingError::InternalServerError(e)
    }
}

impl From<http::Error> for APIRoutingError {
    fn from(e: http::Error) -> Self {
        APIRoutingError::UnprocessableEntity(e.to_string())
    }
}
impl From<serde_json::Error> for APIRoutingError {
    fn from(e: serde_json::Error) -> Self {
        APIRoutingError::UnprocessableEntity(e.to_string())
    }
}

impl std::fmt::Display for APIRoutingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            APIRoutingError::BadRequest(message) =>
                write!(f, "{}", parse_api_routing_error_message(message, "Bad request", "")),
            APIRoutingError::Unauthorized(message) =>
                write!(
                    f,
                    "{}",
                    parse_api_routing_error_message(message, "Unauthorized", "Access denied: ")
                ),
            APIRoutingError::Forbidden(message) =>
                write!(f, "{}", parse_api_routing_error_message(message, "Forbidden", "")),
            APIRoutingError::NotFound(message) =>
                write!(f, "{}", parse_api_routing_error_message(message, "Not found", "")),
            APIRoutingError::UnprocessableEntity(message) =>
                write!(
                    f,
                    "{}",
                    parse_api_routing_error_message(message, "Unauthorized", "Validation error: ")
                ),
            APIRoutingError::InternalServerError(message) =>
                write!(
                    f,
                    "{}",
                    parse_api_routing_error_message(message, "Internal server error", "")
                ),
            APIRoutingError::BadGateway(message) =>
                write!(f, "{}", parse_api_routing_error_message(message, "Bad gateway", "")),
            APIRoutingError::ServiceUnavailable(message) =>
                write!(
                    f,
                    "{}",
                    parse_api_routing_error_message(message, "Service unavailable", "")
                ),
            APIRoutingError::GatewayTimeout(message) =>
                write!(f, "{}", parse_api_routing_error_message(message, "Gateway timeout", "")),
        }
    }
}

impl From<reqwest::Error> for APIRoutingError {
    fn from(e: reqwest::Error) -> Self {
        match e.status().unwrap_or_default() {
            StatusCode::BAD_REQUEST => APIRoutingError::BadRequest(e.to_string()),
            StatusCode::UNAUTHORIZED => APIRoutingError::Unauthorized(e.to_string()),
            StatusCode::FORBIDDEN => APIRoutingError::Forbidden(e.to_string()),
            StatusCode::NOT_FOUND => APIRoutingError::NotFound(e.to_string()),
            StatusCode::UNPROCESSABLE_ENTITY => APIRoutingError::UnprocessableEntity(e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR =>
                APIRoutingError::InternalServerError(e.to_string()),
            StatusCode::BAD_GATEWAY => APIRoutingError::BadGateway(e.to_string()),
            StatusCode::SERVICE_UNAVAILABLE => APIRoutingError::ServiceUnavailable(e.to_string()),
            StatusCode::GATEWAY_TIMEOUT => APIRoutingError::GatewayTimeout(e.to_string()),
            _ => APIRoutingError::InternalServerError(e.to_string()),
        }
    }
}

fn parse_api_routing_error_message(
    message: &String,
    default: impl Into<String>,
    prefix: impl Into<String>
) -> String {
    let mut error_message = default.into();
    if message != "default" {
        error_message = format!("{}{}", prefix.into(), message);
    }
    error_message
}

pub fn resolve_external_service_bad_response(
    mut status_code: StatusCode,
    response_body: String
) -> Result<APIRoutingResponse, APIRoutingError> {
    // Ensure JSON response
    let response_json = serde_json
        ::from_str::<serde_json::Value>(&response_body)
        .unwrap_or(json!({"content": response_body}));

    // Use the status code from the response if it's a valid HTTP status code
    if response_json.is_object() {
        let status = response_json
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if status != "" {
            status_code = StatusCode::from_str(status).unwrap_or(status_code);
        }
    }

    Ok(APIRoutingResponse {
        status_code: status_code,
        body: json!({
            "message": format!("External service responded with a status: {}", status_code).to_string(),
            "data": response_json,
        }).to_string(),
        headers: get_default_headers(),
    })
}