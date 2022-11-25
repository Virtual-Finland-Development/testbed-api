use super::{
    response_types::{APIRoutingResponse, APIRoutingError, ParsedRequest},
};

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
            application::cors_preflight_response(parsed_request).await
        }
        ("GET", "/") => {
            application::index(parsed_request).await
        }
        ("GET", "/docs") => {
            application::docs(parsed_request).await
        }
        ("GET", "/openapi.yml") => {
            application::openapi_spec(parsed_request).await
        }
        ("GET", "/health") => {
            application::health_check(parsed_request).await
        }
        ("POST", "/testbed/reverse-proxy") => {
            testbed::engage_reverse_proxy_request(parsed_request).await
        }
        ("POST", "/testbed/productizers/get-population") => {
            testbed::productizers::figure::get_population(parsed_request).await
        }
        ("POST", "/testbed/productizers/find-job-postings") => {
            testbed::productizers::job::find_job_postings(parsed_request).await
        }
        ("POST", "/testbed/productizers/user-profile") => {
            testbed::productizers::user::fetch_user_profile(parsed_request).await
        }
        _ => {
            application::not_found(parsed_request).await
        }
    }
}
