use crate::api::errors::APIRoutingError;
use crate::api::utils::{APIRoutingResponse, ParsedRequest};

pub mod application;
pub mod testbed;

/**
 * Exec API routing
 */
pub async fn exec_router_request(parsed_request: ParsedRequest) -> APIRoutingResponse {
    match get_router_response(parsed_request).await {
        Ok(response) => return response,
        Err(e) => {
            return APIRoutingResponse::from_routing_error(e);
        }
    }
}

/**
 * API router
 */
pub async fn get_router_response(
    parsed_request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    match (parsed_request.method.as_str(), parsed_request.path.as_str()) {
        ("OPTIONS", _) => {
            return application::cors_preflight_response(parsed_request).await;
        }
        ("GET", "/") => {
            return application::index(parsed_request).await;
        }
        ("GET", "/docs") => {
            return application::docs(parsed_request).await;
        }
        ("GET", "/openapi.yml") => {
            return application::openapi_spec(parsed_request).await;
        }
        ("GET", "/health") => {
            return application::health_check(parsed_request).await;
        }
        ("POST", "/testbed/reverse-proxy") => {
            return testbed::engage_reverse_proxy_request(parsed_request).await;
        }
        ("POST", "/testbed/productizers/get-population") => {
            return testbed::productizers::figure::get_population(parsed_request).await;
        }
        ("POST", "/testbed/productizers/find-job-postings") => {
            return testbed::productizers::job::find_job_postings(parsed_request).await;
        }
        _ => {
            return application::not_found(parsed_request).await;
        }
    }
}
