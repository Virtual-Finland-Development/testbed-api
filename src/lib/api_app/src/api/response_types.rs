use super::utils::get_default_headers;
use http::header::HeaderMap;
use http::StatusCode;
use lambda_http::aws_lambda_events::query_map::QueryMap;
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
            })
            .to_string()
            .as_ref(),
            get_default_headers(),
        )
    }
}

pub struct ParsedRequest {
    pub path: String,
    pub method: String,
    pub query: QueryMap,
    pub headers: HeaderMap,
    pub body: String,
}

/**
 * Exceptions
 */

#[derive(Debug)]
pub enum APIRoutingError {
    // 400
    BadRequest,
    // 401
    Unauthorized(String),
    // 403
    Forbidden,
    // 404
    NotFound,
    // 422
    UnprocessableEntity(String),
    // 500
    InternalServerError,
    // 502
    BadGateway,
    // 503
    ServiceUnavailable,
    // 504
    GatewayTimeout,
}

impl APIRoutingError {
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            APIRoutingError::BadRequest => StatusCode::BAD_REQUEST,
            APIRoutingError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            APIRoutingError::Forbidden => StatusCode::FORBIDDEN,
            APIRoutingError::NotFound => StatusCode::NOT_FOUND,
            APIRoutingError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            APIRoutingError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            APIRoutingError::BadGateway => StatusCode::BAD_GATEWAY,
            APIRoutingError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            APIRoutingError::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

impl std::error::Error for APIRoutingError {}

impl From<std::string::String> for APIRoutingError {
    fn from(_: std::string::String) -> Self {
        APIRoutingError::InternalServerError
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
            APIRoutingError::BadRequest => write!(f, "Bad request"),
            APIRoutingError::Unauthorized(message) => {
                write!(f, "Access denied: {}", message)
            }
            APIRoutingError::Forbidden => write!(f, "Forbidden"),
            APIRoutingError::NotFound => write!(f, "Not found"),
            APIRoutingError::UnprocessableEntity(message) => {
                write!(f, "Validation error: {}", message)
            }
            APIRoutingError::InternalServerError => write!(f, "Internal server error"),
            APIRoutingError::BadGateway => write!(f, "Bad gateway"),
            APIRoutingError::ServiceUnavailable => write!(f, "Service unavailable"),
            APIRoutingError::GatewayTimeout => write!(f, "Gateway timeout"),
        }
    }
}

impl From<reqwest::Error> for APIRoutingError {
    fn from(e: reqwest::Error) -> Self {
        match e.status().unwrap_or_default() {
            StatusCode::BAD_REQUEST => APIRoutingError::BadRequest,
            StatusCode::UNAUTHORIZED => APIRoutingError::Unauthorized(e.to_string()),
            StatusCode::FORBIDDEN => APIRoutingError::Forbidden,
            StatusCode::NOT_FOUND => APIRoutingError::NotFound,
            StatusCode::UNPROCESSABLE_ENTITY => APIRoutingError::UnprocessableEntity(e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => APIRoutingError::InternalServerError,
            StatusCode::BAD_GATEWAY => APIRoutingError::BadGateway,
            StatusCode::SERVICE_UNAVAILABLE => APIRoutingError::ServiceUnavailable,
            StatusCode::GATEWAY_TIMEOUT => APIRoutingError::GatewayTimeout,
            _ => APIRoutingError::InternalServerError,
        }
    }
}
