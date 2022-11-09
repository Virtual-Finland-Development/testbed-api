use http::StatusCode;

#[derive(Debug)]
pub enum APIRoutingError {
    // 400
    BadRequest,
    // 401
    Unauthorized,
    // 403
    Forbidden,
    // 404
    NotFound,
    // 422
    UnprocessableEntity,
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
            APIRoutingError::Unauthorized => StatusCode::UNAUTHORIZED,
            APIRoutingError::Forbidden => StatusCode::FORBIDDEN,
            APIRoutingError::NotFound => StatusCode::NOT_FOUND,
            APIRoutingError::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
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

impl std::fmt::Display for APIRoutingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            APIRoutingError::BadRequest => write!(f, "Bad request"),
            APIRoutingError::Unauthorized => write!(f, "Unauthorized"),
            APIRoutingError::Forbidden => write!(f, "Forbidden"),
            APIRoutingError::NotFound => write!(f, "Not found"),
            APIRoutingError::UnprocessableEntity => write!(f, "Validation error"),
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
            StatusCode::UNAUTHORIZED => APIRoutingError::Unauthorized,
            StatusCode::FORBIDDEN => APIRoutingError::Forbidden,
            StatusCode::NOT_FOUND => APIRoutingError::NotFound,
            StatusCode::UNPROCESSABLE_ENTITY => APIRoutingError::UnprocessableEntity,
            StatusCode::INTERNAL_SERVER_ERROR => APIRoutingError::InternalServerError,
            StatusCode::BAD_GATEWAY => APIRoutingError::BadGateway,
            StatusCode::SERVICE_UNAVAILABLE => APIRoutingError::ServiceUnavailable,
            StatusCode::GATEWAY_TIMEOUT => APIRoutingError::GatewayTimeout,
            _ => APIRoutingError::InternalServerError,
        }
    }
}
